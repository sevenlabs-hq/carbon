//! Watch-list bootstrap + periodic refresh.
//!
//! Stage 1 keeps this simple: snapshot the union of `attack_wallets` and
//! `copytrade_wallets` over Supabase REST every `WATCH_REFRESH_INTERVAL_S`
//! and reconcile against the in-memory state. Reconciliation is set-diff
//! (insert new, delete missing), so a churning watch list converges within
//! one refresh interval — adequate for the observation soak phase. A
//! follow-up PR upgrades to Supabase Realtime subscriptions for sub-second
//! propagation when this binary starts driving real triggers.

use crate::event::UserMatch;
use crate::state;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::time::Duration;

const DEFAULT_REFRESH_S: u64 = 30;

#[derive(Deserialize, Debug)]
struct AttackWalletRow {
    id: String,
    user_id: String,
    group_id: String,
    address: String,
    label: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CopytradeWalletRow {
    id: String,
    user_id: String,
    group_id: String,
    address: String,
    label: Option<String>,
}

pub fn spawn() {
    let interval_s: u64 = env::var("WATCH_REFRESH_INTERVAL_S")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_REFRESH_S);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_s));
        // Tick once immediately to bootstrap.
        loop {
            interval.tick().await;
            if let Err(e) = refresh_once().await {
                log::warn!("watch_list refresh failed: {e}");
            }
        }
    });
}

async fn refresh_once() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let supabase_url = env::var("SUPABASE_URL")?;
    let key = env::var("SUPABASE_SERVICE_ROLE_KEY")?;
    let http = reqwest::Client::new();

    let attack = fetch_attack(&http, &supabase_url, &key).await?;
    let copytrade = fetch_copytrade(&http, &supabase_url, &key).await?;

    // Build the desired-state map: row_id_key → (wallet, UserMatch)
    let mut desired: HashMap<String, (String, &'static str, UserMatch)> = HashMap::new();
    for row in &attack {
        let key = format!("attack:{}", row.id);
        desired.insert(
            key,
            (
                row.address.clone(),
                "attack",
                UserMatch {
                    user_id: row.user_id.clone(),
                    group_id: row.group_id.clone(),
                    engine: "attack",
                    wallet_label: row.label.clone(),
                },
            ),
        );
    }
    for row in &copytrade {
        let key = format!("copytrade:{}", row.id);
        desired.insert(
            key,
            (
                row.address.clone(),
                "copytrade",
                UserMatch {
                    user_id: row.user_id.clone(),
                    group_id: row.group_id.clone(),
                    engine: "copytrade",
                    wallet_label: row.label.clone(),
                },
            ),
        );
    }

    // Insert or update.
    for (id_key, (wallet, engine, m)) in desired.iter() {
        let row_id = id_key.split(':').nth(1).unwrap_or_default();
        state::add(wallet, row_id, engine, m.clone());
    }

    // Delete entries no longer present. Snapshot current ID_TO_WALLET
    // keys, retain those still in `desired`, remove the rest.
    let live_keys: Vec<String> = state::ID_TO_WALLET
        .iter()
        .map(|kv| kv.key().clone())
        .collect();
    for live_key in live_keys {
        if !desired.contains_key(&live_key) {
            let mut split = live_key.splitn(2, ':');
            let engine = split.next().unwrap_or("");
            let row_id = split.next().unwrap_or("");
            let engine_static = match engine {
                "attack" => "attack",
                "copytrade" => "copytrade",
                _ => continue,
            };
            state::remove_by_id(row_id, engine_static);
        }
    }

    log::info!(
        "watch_list refreshed: attack={} copytrade={} watched_wallets={}",
        attack.len(),
        copytrade.len(),
        state::watch_count()
    );
    Ok(())
}

async fn fetch_attack(
    http: &reqwest::Client,
    base: &str,
    key: &str,
) -> Result<Vec<AttackWalletRow>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{base}/rest/v1/attack_wallets?select=id,user_id,group_id,address,label");
    let rows = http
        .get(&url)
        .header("apikey", key)
        .header("Authorization", format!("Bearer {key}"))
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<AttackWalletRow>>()
        .await?;
    Ok(rows)
}

async fn fetch_copytrade(
    http: &reqwest::Client,
    base: &str,
    key: &str,
) -> Result<Vec<CopytradeWalletRow>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{base}/rest/v1/copytrade_wallets?select=id,user_id,group_id,address,label");
    let rows = http
        .get(&url)
        .header("apikey", key)
        .header("Authorization", format!("Bearer {key}"))
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<CopytradeWalletRow>>()
        .await?;
    Ok(rows)
}
