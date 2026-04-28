//! In-memory watch list. The wallet pubkey IS the DashMap key — we never store
//! `wallet_address` as a struct field, the lookup is by it. The `WatchedWallet`
//! struct holds the per-wallet attribution metadata (which user, which target).

use dashmap::DashMap;
use once_cell::sync::Lazy;
use solana_pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct WatchedWallet {
    pub user_id: String,
    pub target_id: String,
    pub target_name: String,
    pub wallet_label: String,
}

/// Keyed by wallet pubkey (base58). Value = attribution metadata.
pub static WATCH: Lazy<DashMap<String, WatchedWallet>> = Lazy::new(DashMap::new);

/// Mirrored to the atlas-ws datasource's `account_include` set. Carbon's
/// HeliusWebsocket re-subscribes when this changes.
pub static ATLAS_WS_ACCOUNTS: Lazy<Arc<RwLock<HashSet<Pubkey>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashSet::new())));

pub fn lookup(wallet_pubkey: &str) -> Option<WatchedWallet> {
    WATCH.get(wallet_pubkey).map(|v| v.clone())
}

pub fn watch_count() -> usize {
    WATCH.len()
}

pub async fn add(wallet_pubkey: String, watched: WatchedWallet) {
    if let Ok(pk) = Pubkey::from_str(&wallet_pubkey) {
        WATCH.insert(wallet_pubkey, watched);
        let mut set = ATLAS_WS_ACCOUNTS.write().await;
        set.insert(pk);
    } else {
        log::warn!("invalid pubkey: {}", wallet_pubkey);
    }
}

pub async fn remove(wallet_pubkey: &str) {
    WATCH.remove(wallet_pubkey);
    if let Ok(pk) = Pubkey::from_str(wallet_pubkey) {
        let mut set = ATLAS_WS_ACCOUNTS.write().await;
        set.remove(&pk);
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
    let mut set = ATLAS_WS_ACCOUNTS.write().await;
    for k in &to_remove {
        if let Ok(pk) = Pubkey::from_str(k) {
            set.remove(&pk);
        }
    }
    to_remove
}
