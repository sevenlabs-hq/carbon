//! Output sink. Stage 1 = observation only:
//!   1. Structured JSON log line (the durable record)
//!   2. Redis publish to `shreds_observation:<userId>` per matched user
//!      (informational; no consumer reads it yet — frankfurt-rust does
//!      not subscribe to this channel).
//!
//! Bounded mpsc + dedicated writer task so processor hot-paths never block
//! on I/O. Channel-full drops the event with a warn — observation-grade
//! eventual consistency.

use crate::event::ObservedTrigger;
use crate::metrics;
use once_cell::sync::OnceCell;
use redis::AsyncCommands;
use std::env;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc;

const CHANNEL_CAPACITY: usize = 4096;

static SENDER: OnceCell<mpsc::Sender<ObservedTrigger>> = OnceCell::new();

/// Non-blocking submit. Drops on full with a warn — never await a slow
/// downstream from a decode hot path.
pub fn submit(t: ObservedTrigger) {
    if let Some(tx) = SENDER.get() {
        match tx.try_send(t) {
            Ok(()) => {
                metrics::TRIGGERS_EMITTED.fetch_add(1, Ordering::Relaxed);
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                log::warn!("writer channel full; dropping observation");
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                log::error!("writer channel closed; observation lost");
            }
        }
    }
}

/// Spawn the writer task. Idempotent; first call wins.
pub fn spawn() {
    let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
    if SENDER.set(tx).is_err() {
        return;
    }
    tokio::spawn(async move { run(rx).await });
}

async fn run(mut rx: mpsc::Receiver<ObservedTrigger>) {
    let redis_conn = match build_redis_conn().await {
        Some(c) => Some(c),
        None => {
            log::warn!("redis publish disabled — log-only observation");
            None
        }
    };

    while let Some(trigger) = rx.recv().await {
        // Always log. Structured JSON via serde — every field on
        // ObservedTrigger lands as a log field. tracing-subscriber's
        // `json` layer (set up in main) renders this consistently.
        if let Ok(payload) = serde_json::to_string(&trigger) {
            log::info!(target: "carbon_shreds_obs", "{payload}");
        }

        // Publish per-user when redis is up. One message per matched
        // user keeps consumers' filtering trivial.
        if let Some(mut conn) = redis_conn.clone() {
            for matched in &trigger.matched_users {
                let channel = format!("shreds_observation:{}", matched.user_id);
                let payload = match serde_json::to_string(&PublishPayload {
                    trigger: &trigger,
                    user: matched,
                }) {
                    Ok(p) => p,
                    Err(e) => {
                        log::error!("serde error on publish: {e}");
                        continue;
                    }
                };
                let res: redis::RedisResult<i64> = conn.publish(&channel, payload).await;
                if let Err(e) = res {
                    log::warn!("redis publish failed (channel={channel}): {e}");
                }
            }
        }
    }
}

#[derive(serde::Serialize)]
struct PublishPayload<'a> {
    trigger: &'a ObservedTrigger,
    user: &'a crate::event::UserMatch,
}

async fn build_redis_conn() -> Option<redis::aio::MultiplexedConnection> {
    let url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    match redis::Client::open(url) {
        Ok(c) => match c.get_multiplexed_async_connection().await {
            Ok(conn) => Some(conn),
            Err(e) => {
                log::error!("redis connect failed: {e}");
                None
            }
        },
        Err(e) => {
            log::error!("redis client open failed: {e}");
            None
        }
    }
}
