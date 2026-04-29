//! In-memory watch list. The wallet pubkey IS the DashMap key — we never store
//! `wallet_address` as a struct field, the lookup is by it. The `WatchedWallet`
//! struct holds the per-wallet attribution metadata (which user, which target).

use dashmap::DashMap;
use once_cell::sync::Lazy;
use solana_pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Notify, RwLock};

#[derive(Clone, Debug)]
pub struct WatchedWallet {
    pub user_id: String,
    pub target_id: String,
    pub target_name: String,
    pub wallet_label: String,
}

/// Keyed by wallet pubkey (base58). Value = attribution metadata.
pub static WATCH: Lazy<DashMap<String, WatchedWallet>> = Lazy::new(DashMap::new);

/// Reverse lookup `surveillance_wallet_sessions.id` → `wallet_address`.
/// Populated on INSERT/UPDATE in watch_list_sync; consulted on DELETE
/// because Supabase Realtime's DELETE payload's `old_record` only
/// includes the primary key, not the full row (REPLICA IDENTITY FULL
/// notwithstanding — Realtime strips non-PK columns).
pub static ID_TO_WALLET: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);

/// Mirrored to the atlas-ws datasource's `account_include` set. Carbon's
/// HeliusWebsocket re-subscribes when `ATLAS_WS_NOTIFY` is fired.
pub static ATLAS_WS_ACCOUNTS: Lazy<Arc<RwLock<HashSet<Pubkey>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashSet::new())));

/// Wake signal for the HeliusWebsocket transaction subscription. Fired
/// from `add()` / `remove()` whenever ATLAS_WS_ACCOUNTS actually changes
/// — the datasource cancels its current `transaction_subscribe` and
/// re-subscribes with the updated `account_include`.
pub static ATLAS_WS_NOTIFY: Lazy<Arc<Notify>> = Lazy::new(|| Arc::new(Notify::new()));

pub fn lookup(wallet_pubkey: &str) -> Option<WatchedWallet> {
    WATCH.get(wallet_pubkey).map(|v| v.clone())
}

pub fn watch_count() -> usize {
    WATCH.len()
}

pub async fn add(wallet_pubkey: String, watched: WatchedWallet) {
    if let Ok(pk) = Pubkey::from_str(&wallet_pubkey) {
        WATCH.insert(wallet_pubkey, watched);
        let inserted = {
            let mut set = ATLAS_WS_ACCOUNTS.write().await;
            set.insert(pk)
        };
        // Only kick the datasource when the set actually changed —
        // refreshing existing wallet metadata (heartbeat updates, etc.)
        // shouldn't churn Helius re-subscribes.
        if inserted {
            ATLAS_WS_NOTIFY.notify_one();
        }
    } else {
        log::warn!("invalid pubkey: {}", wallet_pubkey);
    }
}

pub async fn remove(wallet_pubkey: &str) {
    WATCH.remove(wallet_pubkey);
    if let Ok(pk) = Pubkey::from_str(wallet_pubkey) {
        let removed = {
            let mut set = ATLAS_WS_ACCOUNTS.write().await;
            set.remove(&pk)
        };
        if removed {
            ATLAS_WS_NOTIFY.notify_one();
        }
    }
}

pub async fn remove_target(target_id: &str) -> Vec<String> {
    let to_remove: Vec<_> = WATCH
        .iter()
        .filter_map(|kv| {
            if kv.value().target_id == target_id {
                Some(kv.key().clone())
            } else {
                None
            }
        })
        .collect();
    for k in &to_remove {
        WATCH.remove(k);
    }
    let mut any_removed = false;
    {
        let mut set = ATLAS_WS_ACCOUNTS.write().await;
        for k in &to_remove {
            if let Ok(pk) = Pubkey::from_str(k) {
                if set.remove(&pk) {
                    any_removed = true;
                }
            }
        }
    }
    if any_removed {
        ATLAS_WS_NOTIFY.notify_one();
    }
    to_remove
}
