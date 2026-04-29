//! Per-transaction activity coordinator.
//!
//! Per-program processors (PumpfunWatch, PumpSwapWatch, AggWatch,
//! Transfer*Watch) decode their ixs and `submit` typed candidates here
//! instead of writing events directly. A flusher task drains the
//! signature-keyed buffer after a short quiet window, picks one winner per
//! (wallet, ActivityFamily) using `Activity::precedence` + data richness as
//! tiebreaker, and ships those final events to the writer.
//!
//! This is what unifies our 31 per-program decoders into a tx-level
//! classifier without touching upstream Carbon decoder source. Every
//! benefit a single `Pipeline::transaction(...)` call would have given us
//! is here, plumbed through fan-in: tip-PDA noise dies because tip
//! candidates lose to swap candidates in the Transfer family vs Swap
//! family with cross-family suppression; Anchor event-log echoes die
//! because the AggWatch event-prefix gate already drops them at submit
//! time; inner-CPI splits collapse because all candidates for the same
//! (sig, wallet, family) reduce to one winner.

use crate::event::SurveillanceEventOut;
use crate::state::WatchedWallet;
use crate::taxonomy::{Activity, ActivityFamily};
use crate::writer;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// How long we wait without seeing a new candidate for a signature before
/// we consider the tx "complete" and flush. Carbon delivers all ixs of a
/// tx within a few ms over the WS; 200ms is generous and keeps tail
/// inner-CPI candidates from being missed.
const QUIET_WINDOW: Duration = Duration::from_millis(200);

/// Sweep interval. Polled, not event-driven — cheap.
const SWEEP_INTERVAL: Duration = Duration::from_millis(50);

/// Hard upper bound on time a signature can sit in the buffer regardless
/// of late candidates. Caps memory in pathological cases.
const MAX_BUFFER_AGE: Duration = Duration::from_secs(2);

#[derive(Clone)]
pub struct ActivityCandidate {
    pub activity: Activity,
    /// Carried for future enrichment paths (e.g. emitting derived events
    /// keyed on the watched wallet's user/target metadata) even though the
    /// coordinator currently reads everything it needs from `event`.
    #[allow(dead_code)]
    pub watched: WatchedWallet,
    pub event: SurveillanceEventOut,
}

struct Bucket {
    candidates: Vec<ActivityCandidate>,
    first_seen: Instant,
    last_seen: Instant,
}

static BUFFER: Lazy<DashMap<String, Bucket>> = Lazy::new(DashMap::new);

pub async fn submit(candidate: ActivityCandidate) {
    let sig = candidate.event.signature.clone();
    let now = Instant::now();
    BUFFER
        .entry(sig)
        .and_modify(|b| {
            b.candidates.push(candidate.clone());
            b.last_seen = now;
        })
        .or_insert_with(|| Bucket {
            candidates: vec![candidate],
            first_seen: now,
            last_seen: now,
        });
}

/// Spawn the flusher loop. Idempotent — call once at startup.
pub fn spawn_flusher() {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(SWEEP_INTERVAL);
        loop {
            interval.tick().await;
            sweep_once().await;
        }
    });
}

async fn sweep_once() {
    let now = Instant::now();
    let ready: Vec<String> = BUFFER
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

    for sig in ready {
        if let Some((_, bucket)) = BUFFER.remove(&sig) {
            for event in classify(bucket.candidates) {
                writer::send_event(event).await;
            }
        }
    }
}

/// Reduce all candidates for a signature into final events.
///
/// Group by (wallet, family). Within each group, pick the candidate whose
/// `Activity::precedence` is highest; break ties by data-richness
/// (non-zero `sol_amount + token_amount` beats zeros — i.e. concrete
/// per-program processor output beats AggWatch fallback). Emit one event
/// per (wallet, family) winner. The Fallback family (ProgramActivity) is
/// only emitted if no other family emitted for that wallet, suppressing
/// "used program X" rows when we already have richer activity.
fn classify(candidates: Vec<ActivityCandidate>) -> Vec<SurveillanceEventOut> {
    // wallet → family → best candidate so far
    let mut best: HashMap<String, HashMap<ActivityFamily, ActivityCandidate>> = HashMap::new();

    for c in candidates {
        let wallet = c.event.wallet_address.clone();
        let family = c.activity.family();
        let entry = best.entry(wallet).or_default().entry(family);
        match entry {
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(c);
            }
            std::collections::hash_map::Entry::Occupied(mut o) => {
                if beats(&c, o.get()) {
                    o.insert(c);
                }
            }
        }
    }

    let mut out = Vec::new();
    for (_wallet, families) in best {
        let has_non_fallback = families.keys().any(|f| *f != ActivityFamily::Fallback);
        for (family, cand) in families {
            if family == ActivityFamily::Fallback && has_non_fallback {
                continue;
            }
            out.push(cand.event);
        }
    }
    out
}

/// Tie-break: higher `Activity::precedence` wins; then non-zero amounts
/// beat zero amounts (rich data beats AggWatch coarse fallback).
fn beats(new: &ActivityCandidate, current: &ActivityCandidate) -> bool {
    let np = new.activity.precedence();
    let cp = current.activity.precedence();
    if np != cp {
        return np > cp;
    }
    let n_rich = new.event.sol_amount > 0.0 || new.event.token_amount > 0.0;
    let c_rich = current.event.sol_amount > 0.0 || current.event.token_amount > 0.0;
    if n_rich && !c_rich {
        return true;
    }
    if !n_rich && c_rich {
        return false;
    }
    // Equal — keep current to avoid churn.
    false
}
