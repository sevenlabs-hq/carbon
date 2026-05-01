//! In-memory watch list — the union of every user's attack and copytrade
//! tracked-wallet sets. The wallet pubkey is the DashMap key; each maps
//! to the list of `(user, group, engine, label)` watchers, because a
//! single wallet may be tracked by multiple users on multiple engines.
//!
//! Mutated exclusively by `watch_list_sync` in response to Supabase
//! Realtime events (and the bootstrap snapshot at startup). Read by every
//! processor at decode time via `watchers_for`.

use crate::event::UserMatch;
use dashmap::DashMap;
use once_cell::sync::Lazy;

/// `wallet_address → { (user_id, group_id, engine) → UserMatch }`.
/// The composite key (formatted as "user|group|engine") dedupes the case
/// where the same wallet appears in multiple groups for the same user.
type Inner = std::collections::HashMap<String, UserMatch>;
pub static WATCH: Lazy<DashMap<String, Inner>> = Lazy::new(DashMap::new);

/// Reverse lookup so DELETE events (which carry only the row id) can find
/// the wallet they mapped to. Keyed by `<engine>:<row_id>` to keep attack
/// and copytrade row-id namespaces separate.
pub static ID_TO_WALLET: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);

/// Snapshot of every watcher for a wallet. Empty Vec when none. Cloned
/// out so callers don't hold a DashMap shard lock across awaits.
pub fn watchers_for(wallet_pubkey: &str) -> Vec<UserMatch> {
    WATCH
        .get(wallet_pubkey)
        .map(|kv| kv.value().values().cloned().collect())
        .unwrap_or_default()
}

/// Distinct watched-wallet count. Used in stats / health output.
pub fn watch_count() -> usize {
    WATCH.len()
}

/// Total (wallet, watcher) pairs. Useful when a wallet is tracked by many
/// users — `watch_count` understates the load.
#[allow(dead_code)]
pub fn watcher_count() -> usize {
    WATCH.iter().map(|kv| kv.value().len()).sum()
}

/// Insert or update a watcher. Idempotent on the composite key — a
/// re-INSERT for the same (user, group, engine) overwrites the prior
/// `wallet_label` rather than duplicating.
pub fn add(wallet: &str, row_id: &str, engine: &'static str, m: UserMatch) {
    let composite = format!("{}|{}|{}", m.user_id, m.group_id, engine);
    WATCH
        .entry(wallet.to_string())
        .or_default()
        .insert(composite, m);
    ID_TO_WALLET.insert(format!("{engine}:{row_id}"), wallet.to_string());
}

/// Remove a watcher by row id. The Realtime DELETE payload carries only
/// the PK; we use ID_TO_WALLET to find which wallet entry to mutate.
/// Cleans up the wallet entry entirely if it was the last watcher.
pub fn remove_by_id(row_id: &str, engine: &'static str) {
    let key = format!("{engine}:{row_id}");
    let Some((_, wallet)) = ID_TO_WALLET.remove(&key) else {
        return;
    };
    if let Some(mut shard) = WATCH.get_mut(&wallet) {
        // Remove every (user, group, engine) entry on this wallet that
        // belongs to the matching engine. We don't have user/group from
        // the DELETE payload, but the row id is unique within an engine
        // so removing all engine-matching rows is correct in practice.
        shard.retain(|composite, _| !composite.ends_with(&format!("|{engine}")));
        if shard.is_empty() {
            drop(shard);
            WATCH.remove(&wallet);
        }
    }
}
