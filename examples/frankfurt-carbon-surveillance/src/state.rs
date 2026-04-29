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
            // remove_by_id returns the wallet whether or not it was the
            // last watcher. A wallet is "emptied" iff WATCH no longer
            // contains it after the removal — that's the contract this
            // function reports back to its caller.
            if !WATCH.contains_key(&w) && !emptied_wallets.contains(&w) {
                emptied_wallets.push(w);
            }
        }
    }
    emptied_wallets
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns a Solana base58 pubkey unique per test so concurrent test
    /// runs don't collide in the global WATCH/ATLAS_WS_ACCOUNTS state.
    /// Encodes 32 bytes (mostly zeros) with the high bytes set from
    /// the seed so the resulting strings are valid base58 pubkeys.
    fn unique_wallet(seed: u64) -> String {
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&seed.to_le_bytes());
        // pad with prefix bytes to avoid leading zeros producing short base58
        bytes[8] = 0xff;
        bytes[9] = 0xee;
        Pubkey::new_from_array(bytes).to_string()
    }

    fn watcher(id: &str, user_id: &str) -> WatchedWallet {
        WatchedWallet {
            id: id.into(),
            user_id: user_id.into(),
            target_id: format!("{}:t", user_id),
            target_name: "n".into(),
            wallet_label: "".into(),
        }
    }

    async fn atlas_set_contains(wallet_str: &str) -> bool {
        let pk = Pubkey::from_str(wallet_str).unwrap();
        ATLAS_WS_ACCOUNTS.read().await.contains(&pk)
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn add_inserts_first_watcher_and_registers_atlas_ws() {
        let w = unique_wallet(0xA001);
        add(w.clone(), watcher("row1", "userA")).await;

        assert_eq!(watchers_for(&w).len(), 1);
        assert!(is_watched(&w));
        assert!(atlas_set_contains(&w).await);
        assert_eq!(ID_TO_WALLET.get("row1").map(|v| v.value().clone()), Some(w.clone()));

        // cleanup
        remove_by_id("row1").await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn two_users_on_same_wallet_each_have_their_own_watcher() {
        let w = unique_wallet(0xA002);
        add(w.clone(), watcher("rowA", "userA")).await;
        add(w.clone(), watcher("rowB", "userB")).await;

        let watchers = watchers_for(&w);
        assert_eq!(watchers.len(), 2);
        let user_ids: Vec<&str> = watchers.iter().map(|w| w.user_id.as_str()).collect();
        assert!(user_ids.contains(&"userA"));
        assert!(user_ids.contains(&"userB"));
        // Wallet appears in atlas set exactly once regardless of watcher count.
        assert!(atlas_set_contains(&w).await);

        // cleanup
        remove_by_id("rowA").await;
        remove_by_id("rowB").await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn add_same_id_twice_replaces_metadata_does_not_duplicate() {
        // Idempotency: handling a duplicate Realtime event for the same
        // row id (or backend retry) must not create a phantom second
        // watcher. The metadata is replaced.
        let w = unique_wallet(0xA003);
        add(w.clone(), watcher("rowDup", "userA")).await;
        let mut updated = watcher("rowDup", "userA");
        updated.wallet_label = "renamed".into();
        add(w.clone(), updated).await;

        let watchers = watchers_for(&w);
        assert_eq!(watchers.len(), 1, "duplicate add should NOT add a second watcher");
        assert_eq!(watchers[0].wallet_label, "renamed");

        remove_by_id("rowDup").await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn remove_by_id_with_multiple_watchers_keeps_others() {
        // Penultimate watcher removed: wallet stays in atlas set;
        // remaining watcher still produces events.
        let w = unique_wallet(0xA004);
        add(w.clone(), watcher("rowA", "userA")).await;
        add(w.clone(), watcher("rowB", "userB")).await;

        let res = remove_by_id("rowA").await;
        assert_eq!(res, Some(w.clone()));

        let remaining = watchers_for(&w);
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].user_id, "userB");
        assert!(
            atlas_set_contains(&w).await,
            "wallet must stay in atlas set while any watcher remains"
        );

        remove_by_id("rowB").await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn remove_by_id_last_watcher_drops_wallet_from_atlas() {
        let w = unique_wallet(0xA005);
        add(w.clone(), watcher("rowOnly", "userA")).await;
        assert!(atlas_set_contains(&w).await);

        let res = remove_by_id("rowOnly").await;
        assert_eq!(res, Some(w.clone()));
        assert!(watchers_for(&w).is_empty());
        assert!(!is_watched(&w));
        assert!(!atlas_set_contains(&w).await, "atlas set must lose wallet on last watcher");
        assert!(ID_TO_WALLET.get("rowOnly").is_none());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn remove_by_id_unknown_id_returns_none() {
        // Realtime DELETE for an id we never tracked (e.g. row was
        // claimed by a different server_id) — handler logs and skips.
        let res = remove_by_id("never-seen-id-xyz").await;
        assert!(res.is_none());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn watchers_for_returns_empty_for_unknown_wallet() {
        let res = watchers_for(&unique_wallet(0xA006));
        assert!(res.is_empty());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn watch_count_vs_watcher_count_distinguish_distinct_wallets() {
        let w1 = unique_wallet(0xA007);
        let w2 = unique_wallet(0xA008);
        let baseline_wallets = watch_count();
        let baseline_watchers = watcher_count();

        // Two users on w1, one on w2 → 3 watcher rows, 2 distinct wallets.
        add(w1.clone(), watcher("rowA", "userA")).await;
        add(w1.clone(), watcher("rowB", "userB")).await;
        add(w2.clone(), watcher("rowC", "userC")).await;

        assert_eq!(watch_count() - baseline_wallets, 2);
        assert_eq!(watcher_count() - baseline_watchers, 3);

        remove_by_id("rowA").await;
        remove_by_id("rowB").await;
        remove_by_id("rowC").await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn remove_target_drops_watcher_across_wallets_for_that_target() {
        // Same target_id spans two different wallets (e.g. a multi-wallet
        // target). remove_target must drop both watchers, returning each
        // emptied wallet exactly once.
        let w1 = unique_wallet(0xA009);
        let w2 = unique_wallet(0xA00A);
        let mut a = watcher("rowA", "userA");
        a.target_id = "tgt-shared".into();
        let mut b = watcher("rowB", "userA");
        b.target_id = "tgt-shared".into();
        let mut c = watcher("rowC", "userA");
        c.target_id = "tgt-other".into();
        add(w1.clone(), a).await;
        add(w1.clone(), c).await; // w1 has two watchers, only one matches target
        add(w2.clone(), b).await;

        let emptied = remove_target("tgt-shared").await;
        // w1 still has rowC (different target) → not emptied
        // w2 had only rowB → emptied
        assert!(emptied.contains(&w2));
        assert!(!emptied.contains(&w1));
        // w1's remaining watcher is rowC
        let w1_remaining = watchers_for(&w1);
        assert_eq!(w1_remaining.len(), 1);
        assert_eq!(w1_remaining[0].id, "rowC");

        remove_by_id("rowC").await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn invalid_pubkey_does_not_corrupt_state() {
        let invalid = "not_a_real_pubkey".to_string();
        let baseline_watchers = watcher_count();
        add(invalid.clone(), watcher("invalidRow", "userA")).await;
        // No watcher added (logged warning instead).
        assert_eq!(watcher_count(), baseline_watchers);
        assert!(ID_TO_WALLET.get("invalidRow").is_none());
    }
}
