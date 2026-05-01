//! Per-(signature, signer, decoder family) winner collapse.
//!
//! Carbon's `Pipeline` fans inner-CPI ixs through the same instruction pipes
//! as top-level ixs. For an aggregator-routed swap (Jupiter route → inner
//! pumpfun.Buy), both the JupiterSwapDecoder and the PumpfunDecoder will
//! emit a candidate observation for the same signature + signer. Without
//! collapse, the writer would publish twice for the same trade, throwing
//! off downstream counters and (later, when bridged) double-trigger executions.
//!
//! The flusher buffers candidates per `(signature, signer, family)` for a
//! quiet window (~150ms — shreds deliver ix events within a few ms of the
//! original entry; 150ms is generous tail). On flush we pick one winner per
//! key by data richness (any candidate that has `sol_amount` set beats one
//! that doesn't), then submit the winner to the writer.

use crate::event::ObservedTrigger;
use crate::writer;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::time::{Duration, Instant};

/// Quiet window before a key is considered ready to flush. Generous enough
/// for inner-CPI tails; tight enough to keep the bridge stage's e2e
/// trigger-to-execute latency under a couple hundred ms.
const QUIET_WINDOW: Duration = Duration::from_millis(150);

/// Maximum lifetime of a buffered key regardless of late candidates. Caps
/// memory if some pathological decoder keeps re-firing on a long-lived sig.
const MAX_BUFFER_AGE: Duration = Duration::from_secs(2);

/// Sweep cadence. Polled — cheap.
const SWEEP_INTERVAL: Duration = Duration::from_millis(50);

struct Bucket {
    candidates: Vec<ObservedTrigger>,
    first_seen: Instant,
    last_seen: Instant,
}

static BUFFER: Lazy<DashMap<String, Bucket>> = Lazy::new(DashMap::new);

/// Buffer a candidate. Coordinator picks one winner per collapse_key.
pub fn submit(t: ObservedTrigger) {
    let key = t.collapse_key();
    let now = Instant::now();
    BUFFER
        .entry(key)
        .and_modify(|b| {
            b.candidates.push(t.clone());
            b.last_seen = now;
        })
        .or_insert_with(|| Bucket {
            candidates: vec![t],
            first_seen: now,
            last_seen: now,
        });
}

/// Spawn the flusher. Idempotent at the call-site — main.rs only calls once.
pub fn spawn_flusher() {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(SWEEP_INTERVAL);
        loop {
            interval.tick().await;
            sweep_once();
        }
    });
}

fn sweep_once() {
    let now = Instant::now();
    let ready_keys: Vec<String> = BUFFER
        .iter()
        .filter_map(|kv| {
            let b = kv.value();
            let quiet = now.duration_since(b.last_seen) >= QUIET_WINDOW;
            let aged = now.duration_since(b.first_seen) >= MAX_BUFFER_AGE;
            if quiet || aged {
                Some(kv.key().clone())
            } else {
                None
            }
        })
        .collect();

    for key in ready_keys {
        let Some((_, bucket)) = BUFFER.remove(&key) else {
            continue;
        };
        if let Some(winner) = pick_winner(bucket.candidates) {
            writer::submit(winner);
        }
    }
}

/// Among candidates for a single (sig, signer, family) tuple, the one with
/// `sol_amount` populated wins. Among equally-rich candidates, the first
/// inserted wins (FIFO is stable for processor ordering). Returns None
/// only if the bucket is empty (defensive — submit() always adds at least
/// one entry).
fn pick_winner(mut candidates: Vec<ObservedTrigger>) -> Option<ObservedTrigger> {
    if candidates.is_empty() {
        return None;
    }
    if candidates.len() == 1 {
        return candidates.pop();
    }
    let best_idx = candidates
        .iter()
        .enumerate()
        .max_by_key(|(_, t)| richness_score(t))
        .map(|(i, _)| i)
        .unwrap_or(0);
    Some(candidates.swap_remove(best_idx))
}

/// Higher score = more useful trigger. Tunable: today the only signal is
/// "do we know the sol amount". When aggregator processors learn to surface
/// `pool` and `source_route` consistently, those become tiebreakers too.
fn richness_score(t: &ObservedTrigger) -> u8 {
    let mut s = 0u8;
    if t.sol_amount.is_some() {
        s += 4;
    }
    if t.pool.is_some() {
        s += 2;
    }
    if t.source_route.is_some() {
        s += 1;
    }
    s
}
