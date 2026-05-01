//! Lightweight per-program counters. Atomic-only; no Prometheus formatting
//! is owned by this module — `main.rs::stats_handler` reads the values and
//! serializes JSON for the `/stats` endpoint. Prometheus scraping (if we
//! add it) reads the same JSON.

use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

/// Total tx updates seen by the pipeline (regardless of decoder match).
pub static TX_SEEN: AtomicU64 = AtomicU64::new(0);

/// Triggers emitted to the writer (after coordinator collapse).
pub static TRIGGERS_EMITTED: AtomicU64 = AtomicU64::new(0);

/// Decoder ix matches per program family. Incremented at decode time —
/// before coordinator collapse. The ratio (TRIGGERS_EMITTED / sum of these)
/// shows how aggressively the coordinator dedupes nested ixs.
static DECODED_PER_PROGRAM: Lazy<DashMap<&'static str, AtomicU64>> = Lazy::new(DashMap::new);

/// Triggers skipped because the signer is not in the watch list.
/// Expected to dominate counts; not an error.
pub static SKIPPED_NOT_WATCHED: AtomicU64 = AtomicU64::new(0);

/// Triggers skipped because the signer is on the router/MEV-tip blocklist.
/// Should be near zero in practice — blocklisted wallets shouldn't be in
/// users' watch lists, but defense-in-depth.
pub static SKIPPED_BLOCKLISTED: AtomicU64 = AtomicU64::new(0);

/// Decoder match cases where extraction couldn't surface the (mint, is_buy,
/// sol_amount) tuple. Tells us how complete each processor's extraction is.
static EXTRACTION_FAILED_PER_PROGRAM: Lazy<DashMap<&'static str, AtomicU64>> =
    Lazy::new(DashMap::new);

pub fn inc_decoded(family: &'static str) {
    DECODED_PER_PROGRAM
        .entry(family)
        .or_insert_with(|| AtomicU64::new(0))
        .fetch_add(1, Ordering::Relaxed);
}

pub fn inc_extraction_failed(family: &'static str) {
    EXTRACTION_FAILED_PER_PROGRAM
        .entry(family)
        .or_insert_with(|| AtomicU64::new(0))
        .fetch_add(1, Ordering::Relaxed);
}

/// Snapshot for the `/stats` endpoint. Cheap — just reads atomics.
pub fn snapshot() -> StatsSnapshot {
    let decoded: std::collections::BTreeMap<&'static str, u64> = DECODED_PER_PROGRAM
        .iter()
        .map(|kv| (*kv.key(), kv.value().load(Ordering::Relaxed)))
        .collect();
    let extraction_failed: std::collections::BTreeMap<&'static str, u64> =
        EXTRACTION_FAILED_PER_PROGRAM
            .iter()
            .map(|kv| (*kv.key(), kv.value().load(Ordering::Relaxed)))
            .collect();
    StatsSnapshot {
        tx_seen: TX_SEEN.load(Ordering::Relaxed),
        triggers_emitted: TRIGGERS_EMITTED.load(Ordering::Relaxed),
        skipped_not_watched: SKIPPED_NOT_WATCHED.load(Ordering::Relaxed),
        skipped_blocklisted: SKIPPED_BLOCKLISTED.load(Ordering::Relaxed),
        decoded_per_program: decoded,
        extraction_failed_per_program: extraction_failed,
        watched_wallets: crate::state::watch_count(),
    }
}

#[derive(serde::Serialize)]
pub struct StatsSnapshot {
    pub tx_seen: u64,
    pub triggers_emitted: u64,
    pub skipped_not_watched: u64,
    pub skipped_blocklisted: u64,
    pub decoded_per_program: std::collections::BTreeMap<&'static str, u64>,
    pub extraction_failed_per_program: std::collections::BTreeMap<&'static str, u64>,
    pub watched_wallets: usize,
}
