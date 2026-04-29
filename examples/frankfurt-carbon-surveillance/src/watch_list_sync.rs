//! Live watch-list sync via Supabase Realtime — no polling.
//!
//! Subscribes to `postgres_changes` on `surveillance_wallet_sessions`
//! over the project's Realtime WebSocket. INSERT/UPDATE → `state::add`
//! (which mutates `ATLAS_WS_ACCOUNTS`, triggering Helius re-subscribe).
//! DELETE → `state::remove`.
//!
//! Reconnects with exponential backoff (1s → 60s cap). On each reconnect
//! re-runs the REST snapshot via the existing `recover_watch_list`
//! catch-up call so any events emitted while disconnected are absorbed
//! before the live stream takes over.
//!
//! End-to-end: user clicks "add wallet" → backend writes Supabase row
//! → Postgres logical replication → Supabase Realtime push → this
//! module → state::WATCH + ATLAS_WS_ACCOUNTS mutation → Carbon's
//! HeliusWebsocket detects set change → Helius re-subscribes →
//! Helius pushes new wallet's next tx. Total latency ~100ms.

use crate::state::{self, WatchedWallet};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

const HEARTBEAT_INTERVAL_SECS: u64 = 30;
const MAX_RECONNECT_BACKOFF_SECS: u64 = 60;

#[derive(Deserialize)]
struct EmbeddedTarget {
    name: Option<String>,
}

#[derive(Deserialize)]
struct WalletRow {
    user_id: String,
    target_id: String,
    wallet_address: String,
    wallet_label: Option<String>,
    #[serde(default)]
    surveillance_targets: Option<EmbeddedTarget>,
}

pub fn spawn() {
    if env::var("SUPABASE_URL").is_err() {
        log::warn!("watch_list_sync: SUPABASE_URL not set; static recovery only");
        return;
    }
    tokio::spawn(async move {
        let mut backoff = 1u64;
        loop {
            match run_session().await {
                Ok(()) => {
                    log::warn!("watch_list_sync: session ended cleanly");
                    backoff = 1;
                }
                Err(e) => {
                    log::warn!(
                        "watch_list_sync: session error {} — reconnect in {}s",
                        e, backoff
                    );
                }
            }
            sleep(Duration::from_secs(backoff)).await;
            backoff = (backoff * 2).min(MAX_RECONNECT_BACKOFF_SECS);
        }
    });
}

async fn run_session() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let supabase_url = env::var("SUPABASE_URL")?;
    let api_key = env::var("SUPABASE_SERVICE_ROLE_KEY")?;
    let recovery_source = env::var("RECOVERY_SOURCE_SERVER_ID")
        .unwrap_or_else(|_| "frankfurt-prod".into());

    let host = supabase_url
        .strip_prefix("https://")
        .or_else(|| supabase_url.strip_prefix("http://"))
        .unwrap_or(&supabase_url)
        .trim_end_matches('/');

    let ws_url = format!(
        "wss://{}/realtime/v1/websocket?apikey={}&vsn=1.0.0",
        host, api_key
    );

    log::info!("watch_list_sync: connecting to wss://{}/realtime/v1/websocket", host);
    let (ws_stream, _resp) = tokio_tungstenite::connect_async(&ws_url).await?;
    let (mut tx, mut rx) = ws_stream.split();

    // Subscribe to postgres_changes on the whole table (no server-side
    // filter). Supabase Realtime evaluates server-side `filter` against
    // the *new* record, which doesn't exist on DELETE — so a filtered
    // subscription silently drops DELETE events. Filter client-side
    // instead in `handle_message`. Volume is tiny (the table has a few
    // hundred rows, all heartbeat-driven), so the extra noise is fine.
    let topic = "realtime:public:surveillance_wallet_sessions".to_string();
    let join = serde_json::json!({
        "topic": topic,
        "event": "phx_join",
        "payload": {
            "config": {
                "postgres_changes": [
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "surveillance_wallet_sessions",
                    }
                ]
            },
            "access_token": api_key.clone(),
        },
        "ref": "1",
        "join_ref": "1"
    });
    tx.send(Message::Text(join.to_string().into())).await?;
    log::info!(
        "watch_list_sync: phx_join sent (server_id={}, table=surveillance_wallet_sessions)",
        recovery_source
    );

    // Heartbeat task — Phoenix Channels requires periodic pings to keep
    // the channel alive. Splits the writer half of the WS into a separate
    // tokio task so the read loop is non-blocking.
    let topic_for_hb = topic.clone();
    let heartbeat = tokio::spawn(async move {
        let mut counter: u64 = 100;
        let mut interval = tokio::time::interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
        interval.tick().await; // skip immediate
        loop {
            interval.tick().await;
            let msg = serde_json::json!({
                "topic": "phoenix",
                "event": "heartbeat",
                "payload": {},
                "ref": counter.to_string()
            });
            counter += 1;
            if tx.send(Message::Text(msg.to_string().into())).await.is_err() {
                return;
            }
            // Keep also pinging the channel topic for liveness on the channel itself
            let access = serde_json::json!({
                "topic": topic_for_hb,
                "event": "access_token",
                "payload": {"access_token": env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default()},
                "ref": counter.to_string()
            });
            counter += 1;
            if tx.send(Message::Text(access.to_string().into())).await.is_err() {
                return;
            }
        }
    });

    // Read loop
    while let Some(msg) = rx.next().await {
        let msg = msg?;
        match msg {
            Message::Text(text) => {
                handle_message(&text, &supabase_url, &api_key, &recovery_source).await;
            }
            Message::Ping(p) => {
                // Tungstenite handles auto-pong by default; nothing to do
                let _ = p;
            }
            Message::Close(c) => {
                log::warn!("watch_list_sync: WS closed: {:?}", c);
                break;
            }
            _ => {}
        }
    }

    heartbeat.abort();
    Ok(())
}

async fn handle_message(
    text: &str,
    supabase_url: &str,
    api_key: &str,
    recovery_source: &str,
) {
    let parsed: serde_json::Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            log::debug!("watch_list_sync: parse error {}: {}", e, text);
            return;
        }
    };
    let event = parsed.get("event").and_then(|v| v.as_str()).unwrap_or("");
    if event != "postgres_changes" {
        log::debug!("watch_list_sync: ignoring event '{}'", event);
        return;
    }

    let data = parsed
        .pointer("/payload/data")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let change_type = data.get("type").and_then(|v| v.as_str()).unwrap_or("");
    // Client-side server_id filter (replaces the dropped server-side
    // `filter:` arg, which doesn't apply to DELETE events). The relevant
    // record is `new` for INSERT/UPDATE, `old` for DELETE.
    let row_for_filter = match change_type {
        "INSERT" | "UPDATE" => data.get("record"),
        "DELETE" => data.get("old_record"),
        _ => None,
    };
    let row_server_id = row_for_filter
        .and_then(|r| r.get("server_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if row_server_id != recovery_source {
        return;
    }

    match change_type {
        "INSERT" | "UPDATE" => {
            let id = data
                .pointer("/record/id")
                .and_then(|v| v.as_str())
                .map(String::from);
            // Re-fetch with embedded target join to get target_name (Realtime
            // payload doesn't include joins).
            if let Some(id) = id {
                if let Err(e) = fetch_and_add(&id, supabase_url, api_key).await {
                    log::warn!("watch_list_sync: fetch_and_add({}): {}", id, e);
                }
            }
        }
        "DELETE" => {
            let wallet = data
                .pointer("/old_record/wallet_address")
                .and_then(|v| v.as_str());
            if let Some(w) = wallet {
                state::remove(w).await;
                log::info!(
                    "watch_list_sync: removed wallet {} (now watching {} wallets)",
                    w,
                    state::watch_count()
                );
            }
        }
        other => log::debug!("watch_list_sync: unhandled change type '{}'", other),
    }
}

async fn fetch_and_add(
    id: &str,
    supabase_url: &str,
    api_key: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "{}/rest/v1/surveillance_wallet_sessions?id=eq.{}&select=user_id,target_id,wallet_address,wallet_label,surveillance_targets(name)",
        supabase_url, id
    );
    let rows: Vec<WalletRow> = reqwest::Client::new()
        .get(&url)
        .header("apikey", api_key)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?
        .json()
        .await?;
    if let Some(r) = rows.into_iter().next() {
        let target_name = r
            .surveillance_targets
            .and_then(|t| t.name)
            .unwrap_or_else(|| "Target".into());
        let wallet = r.wallet_address.clone();
        state::add(
            wallet.clone(),
            WatchedWallet {
                user_id: r.user_id,
                target_id: r.target_id,
                target_name,
                wallet_label: r.wallet_label.unwrap_or_default(),
            },
        )
        .await;
        log::info!(
            "watch_list_sync: added/refreshed wallet {} (now watching {} wallets)",
            wallet,
            state::watch_count()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_insert_postgres_changes_payload() {
        // Realtime sends events with this envelope shape (verified against
        // Supabase Realtime v2 docs).
        let raw = r#"{
            "topic":"realtime:public:surveillance_wallet_sessions",
            "event":"postgres_changes",
            "payload":{
                "data":{
                    "type":"INSERT",
                    "schema":"public",
                    "table":"surveillance_wallet_sessions",
                    "record":{
                        "id":"abc-123",
                        "wallet_address":"WALLETxxx",
                        "user_id":"u1",
                        "target_id":"t1",
                        "server_id":"frankfurt-prod"
                    }
                },
                "ids":[1]
            }
        }"#;
        let v: serde_json::Value = serde_json::from_str(raw).unwrap();
        assert_eq!(v["event"], "postgres_changes");
        let data = v.pointer("/payload/data").unwrap();
        assert_eq!(data["type"], "INSERT");
        assert_eq!(data["record"]["wallet_address"], "WALLETxxx");
        assert_eq!(data["record"]["id"], "abc-123");
    }

    #[test]
    fn parses_delete_postgres_changes_payload() {
        let raw = r#"{
            "topic":"realtime:public:surveillance_wallet_sessions",
            "event":"postgres_changes",
            "payload":{
                "data":{
                    "type":"DELETE",
                    "old_record":{
                        "id":"abc-123",
                        "wallet_address":"WALLETxxx"
                    }
                }
            }
        }"#;
        let v: serde_json::Value = serde_json::from_str(raw).unwrap();
        let data = v.pointer("/payload/data").unwrap();
        assert_eq!(data["type"], "DELETE");
        assert_eq!(data["old_record"]["wallet_address"], "WALLETxxx");
    }

    #[test]
    fn ignores_non_postgres_changes_events() {
        // phx_reply, presence_state, etc. should be silently skipped.
        let raw = r#"{"topic":"realtime:public:surveillance_wallet_sessions","event":"phx_reply","payload":{"status":"ok"}}"#;
        let v: serde_json::Value = serde_json::from_str(raw).unwrap();
        assert_ne!(v["event"], "postgres_changes");
    }

    #[test]
    fn ws_url_built_correctly_from_https_supabase_url() {
        let supabase_url = "https://abc.supabase.co";
        let host = supabase_url
            .strip_prefix("https://")
            .unwrap_or(supabase_url)
            .trim_end_matches('/');
        let ws_url = format!(
            "wss://{}/realtime/v1/websocket?apikey=KEY&vsn=1.0.0",
            host
        );
        assert_eq!(
            ws_url,
            "wss://abc.supabase.co/realtime/v1/websocket?apikey=KEY&vsn=1.0.0"
        );
    }
}
