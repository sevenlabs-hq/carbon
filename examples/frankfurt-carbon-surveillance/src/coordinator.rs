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
use crate::writer_clickhouse;
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
            for mut event in classify(bucket.candidates) {
                // Single enrichment point. For any event with a token
                // address, pull cached metadata (symbol/name/image) and
                // populate the dedicated columns. On a cache miss, fire
                // a fire-and-forget DAS getAsset — this event ships
                // unchanged (no critical-path delay) and the next event
                // for the same mint hits cache.
                if let Some(mint) = event.token_address.as_deref() {
                    match crate::token_meta::lookup(mint) {
                        Some(meta) => {
                            if event.token_symbol.is_none() {
                                event.token_symbol = meta.symbol;
                            }
                            if event.token_name.is_none() {
                                event.token_name = meta.name;
                            }
                            if event.token_image.is_none() {
                                event.token_image = meta.image;
                            }
                            // Even on a partial-from-Create hit, kick a
                            // DAS fetch to fill in the image. spawn_das
                            // is a no-op if `fetched=true` already.
                            if !meta.fetched {
                                crate::token_meta::spawn_das_fetch(mint.to_string());
                            }
                        }
                        None => {
                            crate::token_meta::spawn_das_fetch(mint.to_string());
                        }
                    }
                }
                // Fan to both writers. Independent failure domains —
                // writer_clickhouse no-ops if CLICKHOUSE_URL isn't set, so
                // this is also the env-gated kill switch for analytics
                // shipping without a code change.
                crate::metrics::inc_events_emitted();
                writer_clickhouse::send_event(event.clone()).await;
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
    // (user_id, wallet) → family → best candidate so far. The user_id
    // is part of the key so two users watching the same wallet each get
    // their own row — collapsing only happens within a single user's
    // view of one wallet.
    let mut best: HashMap<(String, String), HashMap<ActivityFamily, ActivityCandidate>> =
        HashMap::new();

    for c in candidates {
        let key = (c.event.user_id.clone(), c.event.wallet_address.clone());
        let family = c.activity.family();
        let entry = best.entry(key).or_default().entry(family);
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
    for (_key, families) in best {
        // Cross-family suppression rules:
        //
        // - `Fallback` (ProgramActivity) is the coarse "wallet used program X"
        //   row. Any concrete family (Swap, Mint, Liquidity, Transfer, …)
        //   already implies that, so drop Fallback when anything else fired.
        //
        // - `Transfer` is ancillary to anything richer: a swap-tx's outer
        //   SOL transfer (Jito tip / Photon platform fee / WSOL wrap) is
        //   not a user transfer, it's part of the swap. Drop Transfer when
        //   any non-Transfer, non-Fallback family also fired for the
        //   wallet. Standalone transfers (where no swap/mint/etc.
        //   happened) still emit.
        //
        // - Other families (Swap + Mint on a create-and-buy, Liquidity +
        //   Swap, etc.) co-emit — they describe distinct user actions.
        let richer_present = families
            .keys()
            .any(|f| !matches!(f, ActivityFamily::Fallback | ActivityFamily::Transfer));
        let any_concrete_present = families
            .keys()
            .any(|f| !matches!(f, ActivityFamily::Fallback));
        for (family, cand) in families {
            match family {
                ActivityFamily::Fallback if any_concrete_present => continue,
                ActivityFamily::Transfer if richer_present => continue,
                _ => {}
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WatchedWallet;

    fn cand(activity: Activity, wallet: &str, program: &'static str, sol: f64) -> ActivityCandidate {
        cand_with_user(activity, wallet, program, sol, "u")
    }

    fn cand_with_user(
        activity: Activity,
        wallet: &str,
        program: &'static str,
        sol: f64,
        user_id: &str,
    ) -> ActivityCandidate {
        ActivityCandidate {
            activity,
            watched: WatchedWallet {
                id: format!("{}:{}", user_id, wallet),
                user_id: user_id.into(),
                target_id: "t".into(),
                target_name: "n".into(),
                wallet_label: "".into(),
            },
            event: SurveillanceEventOut {
                user_id: user_id.into(),
                target_id: "t".into(),
                target_name: "n".into(),
                wallet_address: wallet.into(),
                wallet_label: "".into(),
                signature: "sig".into(),
                event_type: activity.as_event_type(),
                token_address: None,
                token_symbol: None,
                token_name: None,
                token_image: None,
                sol_amount: sol,
                token_amount: 0.0,
                price_sol: 0.0,
                program,
                counterparty: "".into(),
                block_time: "2026-04-29T00:00:00Z".into(),
                slot: 0,
                raw_data: serde_json::json!({}),
            },
        }
    }

    fn types(events: &[SurveillanceEventOut]) -> Vec<(&str, &str)> {
        let mut v: Vec<_> = events
            .iter()
            .map(|e| (e.event_type, e.program))
            .collect();
        v.sort();
        v
    }

    #[test]
    fn lone_swap_emits_one() {
        let out = classify(vec![cand(Activity::SwapBuy, "A", "pumpfun", 0.5)]);
        assert_eq!(types(&out), vec![("swap_buy", "pumpfun")]);
    }

    #[test]
    fn lone_transfer_emits_one() {
        let out = classify(vec![cand(Activity::TransferOut, "A", "system_program", 0.001)]);
        assert_eq!(types(&out), vec![("transfer_out", "system_program")]);
    }

    #[test]
    fn swap_plus_tip_transfer_same_wallet_suppresses_transfer() {
        // The exact pattern blocking parity: PumpFun swap with an outer
        // SystemProgram tip ix on the same tx, same wallet.
        let out = classify(vec![
            cand(Activity::SwapBuy, "A", "pumpfun", 0.5),
            cand(Activity::TransferOut, "A", "system_program", 0.001),
        ]);
        assert_eq!(types(&out), vec![("swap_buy", "pumpfun")]);
    }

    #[test]
    fn mint_plus_swap_co_emit() {
        // pump.fun create_and_buy: distinct user actions, both surface.
        let out = classify(vec![
            cand(Activity::MintCreate, "A", "pumpfun", 0.0),
            cand(Activity::SwapBuy, "A", "pumpfun", 0.5),
        ]);
        assert_eq!(
            types(&out),
            vec![("mint_create", "pumpfun"), ("swap_buy", "pumpfun")]
        );
    }

    #[test]
    fn fallback_alone_emits() {
        let out = classify(vec![cand(Activity::ProgramActivity, "A", "dflow", 0.0)]);
        assert_eq!(types(&out), vec![("program_activity", "dflow")]);
    }

    #[test]
    fn fallback_suppressed_by_concrete_family() {
        let out = classify(vec![
            cand(Activity::ProgramActivity, "A", "raydium_clmm", 0.0),
            cand(Activity::SwapBuy, "A", "pumpfun", 0.5),
        ]);
        assert_eq!(types(&out), vec![("swap_buy", "pumpfun")]);
    }

    #[test]
    fn fallback_suppressed_by_transfer_too() {
        // Even Transfer is "concrete enough" to imply the program was used.
        let out = classify(vec![
            cand(Activity::ProgramActivity, "A", "memo", 0.0),
            cand(Activity::TransferOut, "A", "system_program", 0.001),
        ]);
        assert_eq!(types(&out), vec![("transfer_out", "system_program")]);
    }

    #[test]
    fn different_wallets_independent() {
        // Swap by wallet A + transfer by wallet B on the same tx → both emit.
        // The cross-family suppression is per-wallet, not per-tx.
        let out = classify(vec![
            cand(Activity::SwapBuy, "A", "pumpfun", 0.5),
            cand(Activity::TransferOut, "B", "system_program", 0.01),
        ]);
        assert_eq!(
            types(&out),
            vec![("swap_buy", "pumpfun"), ("transfer_out", "system_program")]
        );
    }

    #[test]
    fn two_users_watching_same_wallet_each_get_a_row() {
        // Multi-user: user A and user B both watch wallet "W". A swap
        // by W must produce TWO rows (one per user), not one.
        let out = classify(vec![
            cand_with_user(Activity::SwapBuy, "W", "pumpfun", 0.5, "userA"),
            cand_with_user(Activity::SwapBuy, "W", "pumpfun", 0.5, "userB"),
        ]);
        assert_eq!(out.len(), 2);
        let user_ids: Vec<&str> = out.iter().map(|e| e.user_id.as_str()).collect();
        assert!(user_ids.contains(&"userA"));
        assert!(user_ids.contains(&"userB"));
    }

    #[test]
    fn same_user_same_wallet_same_family_collapses_to_one() {
        // Single user, wallet had multi-program swap legs (e.g. pumpfun
        // + AggWatch fallback both fired). Within a user's view we
        // collapse to one row per family.
        let out = classify(vec![
            cand_with_user(Activity::SwapBuy, "W", "pumpfun", 0.5, "userA"),
            cand_with_user(Activity::ProgramActivity, "W", "raydium_clmm", 0.0, "userA"),
        ]);
        // ProgramActivity (Fallback) suppressed by concrete Swap.
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].user_id, "userA");
        assert_eq!(out[0].event_type, "swap_buy");
    }

    #[test]
    fn duplicate_swap_collapses_picking_richer() {
        // AggWatch fallback fired alongside concrete decoder for the same
        // family — coordinator picks richer (non-zero amounts) within the
        // family, then cross-family suppression doesn't trigger.
        let mut a = cand(Activity::SwapBuy, "A", "raydium_launchpad", 0.0);
        a.event.token_amount = 0.0;
        let b = cand(Activity::SwapBuy, "A", "pumpfun", 0.5);
        let out = classify(vec![a, b]);
        assert_eq!(types(&out), vec![("swap_buy", "pumpfun")]);
    }
}
