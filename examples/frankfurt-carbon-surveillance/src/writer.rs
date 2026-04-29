//! Bounded mpsc → batched writer to Supabase + per-event Redis publish.
//!
//! `OUTPUT_MODE` env var controls behavior:
//!   - "parity"      (default during soak): write to carbon_surveillance_parity_events
//!                   table. NO Redis publish — parity table is for offline diff
//!                   vs frankfurt-node's surveillance_events.
//!   - "production"  (after soak + cut-over): write to surveillance_events table
//!                   AND publish to Redis user:<userId>:events.
//!
//! Switch is a single env var change; no code redeploy.

use crate::event::SurveillanceEventOut;
use once_cell::sync::OnceCell;
use redis::AsyncCommands;
use serde::Serialize;
use std::env;
use std::time::Duration;
use tokio::sync::mpsc;

const CHANNEL_CAPACITY: usize = 1000;
const FLUSH_INTERVAL_MS: u64 = 200;
const FLUSH_BATCH_SIZE: usize = 100;

static SENDER: OnceCell<mpsc::Sender<SurveillanceEventOut>> = OnceCell::new();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputMode {
    Parity,
    Production,
}

impl OutputMode {
    fn from_env() -> Self {
        match env::var("OUTPUT_MODE").unwrap_or_else(|_| "parity".into()).as_str() {
            "production" | "prod" => OutputMode::Production,
            _ => OutputMode::Parity,
        }
    }

    fn table(&self) -> &'static str {
        match self {
            OutputMode::Parity => "carbon_surveillance_parity_events",
            OutputMode::Production => "surveillance_events",
        }
    }

    fn publish_to_redis(&self) -> bool {
        matches!(self, OutputMode::Production)
    }
}

pub async fn send_event(event: SurveillanceEventOut) {
    if let Some(tx) = SENDER.get() {
        if let Err(e) = tx.try_send(event) {
            // Drop on full — surveillance is eventual-consistency, log + continue
            log::warn!("event channel full or closed: {}", e);
        }
    }
}

pub fn spawn() {
    let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
    SENDER.set(tx).ok();
    tokio::spawn(async move { writer_loop(rx).await });
}

async fn writer_loop(mut rx: mpsc::Receiver<SurveillanceEventOut>) {
    let mode = OutputMode::from_env();
    log::info!("writer starting in {:?} mode (table = {})", mode, mode.table());

    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let supabase_key =
        env::var("SUPABASE_SERVICE_ROLE_KEY").expect("SUPABASE_SERVICE_ROLE_KEY not set");
    let http = reqwest::Client::new();

    let redis_conn = if mode.publish_to_redis() {
        let url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
        match redis::Client::open(url) {
            Ok(c) => match c.get_multiplexed_async_connection().await {
                Ok(conn) => Some(conn),
                Err(e) => {
                    log::error!("redis connect failed: {}", e);
                    None
                }
            },
            Err(e) => {
                log::error!("redis client open failed: {}", e);
                None
            }
        }
    } else {
        None
    };

    let mut batch: Vec<SurveillanceEventOut> = Vec::with_capacity(FLUSH_BATCH_SIZE);
    let flush_interval = Duration::from_millis(FLUSH_INTERVAL_MS);

    loop {
        let collect_deadline = tokio::time::sleep(flush_interval);
        tokio::pin!(collect_deadline);
        loop {
            tokio::select! {
                biased;
                maybe_event = rx.recv() => match maybe_event {
                    Some(event) => {
                        batch.push(event);
                        if batch.len() >= FLUSH_BATCH_SIZE { break; }
                    }
                    None => return, // channel closed
                },
                _ = &mut collect_deadline => break,
            }
        }
        if batch.is_empty() {
            continue;
        }

        if mode.publish_to_redis() {
            if let Some(mut conn) = redis_conn.clone() {
                for event in &batch {
                    let channel = format!("user:{}:events", event.user_id);
                    // Frontend expects {event:"event", data: {...}} envelope —
                    // matches frankfurt-node's publishEvent() shape.
                    let envelope = serde_json::json!({
                        "event": "event",
                        "data": event,
                        "ts": chrono::Utc::now().timestamp_millis(),
                    });
                    let payload = match serde_json::to_string(&envelope) {
                        Ok(s) => s,
                        Err(e) => {
                            log::warn!("serialize event: {}", e);
                            continue;
                        }
                    };
                    let _: redis::RedisResult<i64> = conn.publish(&channel, &payload).await;
                }
            }
        }

        // Batch INSERT to Supabase
        let rows: Vec<DbRow> =
            batch.iter().map(|e| DbRow::from_event(e)).collect();
        let url = format!("{}/rest/v1/{}", supabase_url, mode.table());
        match http
            .post(&url)
            .header("apikey", &supabase_key)
            .header("Authorization", format!("Bearer {}", supabase_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal,resolution=ignore-duplicates")
            .json(&rows)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {}
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                log::warn!(
                    "supabase batch insert non-2xx ({}): {}",
                    status,
                    &body[..body.len().min(300)]
                );
            }
            Err(e) => log::warn!("supabase batch insert error: {}", e),
        }

        batch.clear();
    }
}

#[derive(Serialize)]
struct DbRow<'a> {
    user_id: &'a str,
    target_id: &'a str,
    wallet_address: &'a str,
    signature: &'a str,
    event_type: &'a str,
    token_address: Option<&'a str>,
    token_symbol: Option<&'a str>,
    token_name: Option<&'a str>,
    token_image: Option<&'a str>,
    sol_amount: f64,
    token_amount: f64,
    price_sol: f64,
    program_id: &'a str,
    counterparty: &'a str,
    raw_data: &'a serde_json::Value,
    block_time: &'a str,
}

impl<'a> DbRow<'a> {
    fn from_event(e: &'a SurveillanceEventOut) -> Self {
        DbRow {
            user_id: &e.user_id,
            target_id: &e.target_id,
            wallet_address: &e.wallet_address,
            signature: &e.signature,
            event_type: e.event_type,
            token_address: e.token_address.as_deref(),
            token_symbol: e.token_symbol.as_deref(),
            token_name: e.token_name.as_deref(),
            token_image: e.token_image.as_deref(),
            sol_amount: e.sol_amount,
            token_amount: e.token_amount,
            price_sol: e.price_sol,
            program_id: e.program,
            counterparty: &e.counterparty,
            raw_data: &e.raw_data,
            block_time: &e.block_time,
        }
    }
}
