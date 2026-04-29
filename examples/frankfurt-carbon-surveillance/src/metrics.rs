//! Minimum-viable Prometheus metrics. Exposed at `/metrics` on the
//! existing control HTTP server. Scraped by the VPS's Prometheus
//! instance; alerts on absence of scrape (Carbon down) live in
//! `prometheus.alert_rules.yml`.
//!
//! Counters use `AtomicU64::Relaxed` because metrics are best-effort
//! observability — we don't need cross-thread happens-before ordering
//! for accurate scrape results, and Relaxed avoids fences in hot paths.

use std::sync::atomic::{AtomicU64, Ordering};

/// Total events emitted to the writer (post-coordinator). Includes
/// every fan-out row — a multi-watcher swap that produces 2 rows
/// counts as 2.
pub static EVENTS_EMITTED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Total Helius transaction-subscribe re-subscribe operations. Should
/// stay in low single digits per minute under normal churn; spikes
/// indicate watch-list thrash that the debounce should be absorbing.
pub static HELIUS_RESUBS_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Sum of "coalesced N notifies" beyond the first, across all bursts.
/// Debounce effectiveness = coalesced / (resubs + coalesced).
pub static HELIUS_RESUB_NOTIFIES_COALESCED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Realtime postgres_changes events received (INSERT/UPDATE/DELETE
/// combined). Sudden flatline = Realtime WS dropped silently.
pub static REALTIME_EVENTS_TOTAL: AtomicU64 = AtomicU64::new(0);

/// ClickHouse insert batch failures. Should be 0; non-zero indicates
/// auth/network/schema drift.
pub static CLICKHOUSE_INSERT_FAILURES_TOTAL: AtomicU64 = AtomicU64::new(0);

#[inline]
pub fn inc_events_emitted() {
    EVENTS_EMITTED_TOTAL.fetch_add(1, Ordering::Relaxed);
}
// `inc_helius_resubs` and `add_helius_resub_coalesced` are intentionally
// not yet wired — the resub loop lives in the upstream
// `carbon-helius-atlas-ws-datasource` crate and instrumenting it would
// require crossing the crate boundary. The counters and exposition
// rendering exist now so we can wire them in a follow-up without
// touching the metrics surface again.
#[allow(dead_code)]
#[inline]
pub fn inc_helius_resubs() {
    HELIUS_RESUBS_TOTAL.fetch_add(1, Ordering::Relaxed);
}
#[allow(dead_code)]
#[inline]
pub fn add_helius_resub_coalesced(n: u64) {
    HELIUS_RESUB_NOTIFIES_COALESCED_TOTAL.fetch_add(n, Ordering::Relaxed);
}
#[inline]
pub fn inc_realtime_events() {
    REALTIME_EVENTS_TOTAL.fetch_add(1, Ordering::Relaxed);
}
#[inline]
pub fn inc_clickhouse_failures() {
    CLICKHOUSE_INSERT_FAILURES_TOTAL.fetch_add(1, Ordering::Relaxed);
}

/// Render Prometheus exposition format. Read counters with Relaxed —
/// scrape is approximate and atomicity per-counter is enough.
pub fn render(watch_wallets: usize, watcher_rows: usize, atlas_set_size: usize) -> String {
    let events = EVENTS_EMITTED_TOTAL.load(Ordering::Relaxed);
    let resubs = HELIUS_RESUBS_TOTAL.load(Ordering::Relaxed);
    let coalesced = HELIUS_RESUB_NOTIFIES_COALESCED_TOTAL.load(Ordering::Relaxed);
    let realtime = REALTIME_EVENTS_TOTAL.load(Ordering::Relaxed);
    let ch_fail = CLICKHOUSE_INSERT_FAILURES_TOTAL.load(Ordering::Relaxed);
    format!(
        "# HELP frankfurt_carbon_watch_wallets Distinct wallets currently watched.\n\
# TYPE frankfurt_carbon_watch_wallets gauge\n\
frankfurt_carbon_watch_wallets {}\n\
# HELP frankfurt_carbon_watcher_rows Total (wallet, watcher) pairs — multi-watcher rows count individually.\n\
# TYPE frankfurt_carbon_watcher_rows gauge\n\
frankfurt_carbon_watcher_rows {}\n\
# HELP frankfurt_carbon_atlas_ws_account_include Wallets in the Helius transaction_subscribe filter.\n\
# TYPE frankfurt_carbon_atlas_ws_account_include gauge\n\
frankfurt_carbon_atlas_ws_account_include {}\n\
# HELP frankfurt_carbon_events_emitted_total Events emitted to the writer (post-coordinator).\n\
# TYPE frankfurt_carbon_events_emitted_total counter\n\
frankfurt_carbon_events_emitted_total {}\n\
# HELP frankfurt_carbon_helius_resubs_total Helius transaction_subscribe re-subscribes.\n\
# TYPE frankfurt_carbon_helius_resubs_total counter\n\
frankfurt_carbon_helius_resubs_total {}\n\
# HELP frankfurt_carbon_helius_resub_notifies_coalesced_total Notifies absorbed by debounce beyond the first per burst.\n\
# TYPE frankfurt_carbon_helius_resub_notifies_coalesced_total counter\n\
frankfurt_carbon_helius_resub_notifies_coalesced_total {}\n\
# HELP frankfurt_carbon_realtime_events_total Supabase Realtime postgres_changes events received.\n\
# TYPE frankfurt_carbon_realtime_events_total counter\n\
frankfurt_carbon_realtime_events_total {}\n\
# HELP frankfurt_carbon_clickhouse_insert_failures_total ClickHouse insert batch failures (non-2xx).\n\
# TYPE frankfurt_carbon_clickhouse_insert_failures_total counter\n\
frankfurt_carbon_clickhouse_insert_failures_total {}\n",
        watch_wallets, watcher_rows, atlas_set_size,
        events, resubs, coalesced, realtime, ch_fail,
    )
}
