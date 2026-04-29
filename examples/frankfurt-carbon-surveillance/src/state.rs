//! In-memory watch list. The wallet pubkey IS the DashMap key — we never store
//! `wallet_address` as a struct field, the lookup is by it. Each wallet maps
//! to a HashMap of `surveillance_wallet_sessions.id → WatchedWallet`, so
//! multiple users can watch the same wallet (the famous-trader-target case)
//! and each gets their own attribution metadata + event row.

use dashmap::DashMap;
use once_cell::sync::Lazy;
use solana_pubkey::Pubkey;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Notify, RwLock};

#[derive(Clone, Debug)]
pub struct WatchedWallet {
    /// `surveillance_wallet_sessions.id` — primary key in Supabase. Unique
    /// per session row even if the same (user, target, wallet) tuple
    /// repeats (e.g. stop+restart). Used as the inner-map dedup key.
    pub id: String,
    pub user_id: String,
    pub target_id: String,
    pub target_name: String,
    pub wallet_label: String,
}

/// `wallet_address → { row_id → WatchedWallet }`. Multiple watchers per
/// wallet for the multi-user case.
pub static WATCH: Lazy<DashMap<String, HashMap<String, WatchedWallet>>> =
    Lazy::new(DashMap::new);

/// Reverse lookup `surveillance_wallet_sessions.id → wallet_address`.
/// Populated by `add()`, consulted by `remove_by_id()` because Supabase
/// Realtime's DELETE payload's `old_record` only includes the PK, not
/// the full row.
pub static ID_TO_WALLET: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);

/// Mirrored to the atlas-ws datasource's `account_include` set. A wallet
/// is in this set iff at least one watcher exists for it.
pub static ATLAS_WS_ACCOUNTS: Lazy<Arc<RwLock<HashSet<Pubkey>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashSet::new())));

/// Wake signal for the HeliusWebsocket transaction subscription. Fired
/// from `add()` / `remove_by_id()` whenever ATLAS_WS_ACCOUNTS actually
/// changes — the datasource cancels its current `transaction_subscribe`
/// and re-subscribes with the updated `account_include`.
pub static ATLAS_WS_NOTIFY: Lazy<Arc<Notify>> = Lazy::new(|| Arc::new(Notify::new()));

/// All current watchers for a wallet. Empty Vec if none. Cloned out so
/// callers don't hold a DashMap shard lock across awaits.
pub fn watchers_for(wallet_pubkey: &str) -> Vec<WatchedWallet> {
    WATCH
        .get(wallet_pubkey)
        .map(|kv| kv.value().values().cloned().collect())
        .unwrap_or_default()
}

/// True iff at least one watcher exists. Cheap presence check used by
/// AggWatch's account scan.
pub fn is_watched(wallet_pubkey: &str) -> bool {
    WATCH
        .get(wallet_pubkey)
        .map(|kv| !kv.value().is_empty())
        .unwrap_or(false)
}

/// Number of distinct wallets being watched. Multiple watchers per
/// wallet count as one. Used in human-facing log messages.
pub fn watch_count() -> usize {
    WATCH.len()
}

/// Number of (wallet, watcher) pairs. Used to size the
/// runtime story when counting watchers vs wallets matters.
#[allow(dead_code)]
pub fn watcher_count() -> usize {
    WATCH.iter().map(|kv| kv.value().len()).sum()
}

/// Insert/update a watcher for a wallet. Idempotent on (wallet, row_id):
/// re-running with the same id replaces the existing watcher metadata.
/// Fires `ATLAS_WS_NOTIFY` only when the wallet is newly observed (so
/// metadata-refresh updates don't churn Helius re-subscribes).
pub async fn add(wallet_pubkey: String, watched: WatchedWallet) {
    let pk = match Pubkey::from_str(&wallet_pubkey) {
        Ok(pk) => pk,
        Err(_) => {
            log::warn!("invalid pubkey: {}", wallet_pubkey);
            return;
        }
    };
    let row_id = watched.id.clone();
    ID_TO_WALLET.insert(row_id.clone(), wallet_pubkey.clone());

    // Insert into the per-wallet watcher map. DashMap's entry API
    // takes a shard lock per-key; we're not holding it across awaits.
    WATCH
        .entry(wallet_pubkey.clone())
        .or_default()
        .insert(row_id, watched);

    let inserted = {
        let mut set = ATLAS_WS_ACCOUNTS.write().await;
        set.insert(pk)
    };
    if inserted {
        ATLAS_WS_NOTIFY.notify_one();
    }
}

/// Remove a single watcher by row id. Returns the wallet address it
/// was attached to. If this was the last watcher for the wallet, the
/// wallet is dropped from `ATLAS_WS_ACCOUNTS` and Helius re-subscribes
/// with a smaller `account_include`.
pub async fn remove_by_id(row_id: &str) -> Option<String> {
    let wallet = ID_TO_WALLET.remove(row_id).map(|(_, w)| w)?;

    // Remove the (wallet, row_id) → WatchedWallet entry. Track whether
    // that emptied the inner map so we know whether to also drop the
    // wallet from ATLAS_WS_ACCOUNTS.
    let now_empty = match WATCH.get_mut(&wallet) {
        Some(mut entry) => {
            entry.remove(row_id);
            entry.is_empty()
        }
        None => false,
    };
    if now_empty {
        WATCH.remove(&wallet);
        if let Ok(pk) = Pubkey::from_str(&wallet) {
            let removed = {
                let mut set = ATLAS_WS_ACCOUNTS.write().await;
                set.remove(&pk)
            };
            if removed {
                ATLAS_WS_NOTIFY.notify_one();
            }
        }
    }
    Some(wallet)
}

/// Remove all watchers for a target_id (across all wallets). Used by
/// the `/surveillance/target` HTTP endpoint when a user retires a target
/// entirely. Returns the list of wallets that lost their LAST watcher
/// (so callers know what was removed from Helius's filter); intermediate
/// removals where the wallet still has other watchers don't appear.
#[allow(dead_code)]
pub async fn remove_target(target_id: &str) -> Vec<String> {
    // Collect (row_id, wallet_address) pairs that match this target.
    let to_remove: Vec<(String, String)> = WATCH
        .iter()
        .flat_map(|kv| {
            let wallet = kv.key().clone();
            kv.value()
                .iter()
                .filter_map(|(row_id, w)| {
                    if w.target_id == target_id {
                        Some((row_id.clone(), wallet.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut emptied_wallets = Vec::new();
    for (row_id, _wallet) in &to_remove {
        if let Some(w) = remove_by_id(row_id).await {
            // remove_by_id only returns the wallet — we report it once
            // even if multiple watchers matched, so dedupe via a set.
            if !emptied_wallets.contains(&w) {
                emptied_wallets.push(w);
            }
        }
    }
    emptied_wallets
}
