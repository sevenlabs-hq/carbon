//! `ObservedTrigger` — the canonical shape every processor emits when a
//! tracked wallet's swap is decoded off the shred firehose.
//!
//! Stage 1 is observation only: this struct flows from processor → coordinator
//! → writer (log + Redis publish). It carries everything an executor would
//! need later, so when the bridge stage lands the data shape doesn't change.

use serde::Serialize;

/// One observed swap from a wallet a user is tracking. Emitted exactly once
/// per (signature, wallet, decoder family) — the coordinator collapses
/// duplicates that arise when nested decoders match the same tx.
#[derive(Clone, Debug, Serialize)]
pub struct ObservedTrigger {
    // ── Provenance ─────────────────────────────────────────────────
    /// Solana transaction signature. Idempotency key for downstream consumers.
    pub signature: String,
    /// Slot the leader produced the shred entry in.
    pub slot: u64,
    /// Server-side receive-time in ms-since-epoch. Shred-mode does not have
    /// a real `block_time` — this is the moment we decoded it.
    pub observed_ts_ms: i64,

    // ── Trade semantics ────────────────────────────────────────────
    /// Tracked wallet (the target the user is mirroring). Pubkey base58.
    pub signer: String,
    /// Token mint being traded.
    pub mint: String,
    /// True for buy (user wants to copy), false for sell.
    pub is_buy: bool,
    /// SOL amount on the trade. `None` when the decoder cannot establish it
    /// from a single ix (e.g. multi-leg routes); downstream can ignore the
    /// trigger or derive from another source.
    pub sol_amount: Option<f64>,

    // ── Routing context ────────────────────────────────────────────
    /// Decoder family that fired this trigger: "pumpfun" | "pumpswap" |
    /// "jupiter" | "dflow" | "raydium_amm_v4" | … . Stable identifier;
    /// processors define the canonical string per program.
    pub decoder_family: &'static str,
    /// The underlying execution venue when the top-level was an aggregator.
    /// `None` when the top-level decoder IS the venue (e.g. a direct
    /// pumpfun.Buy emits `decoder_family="pumpfun"`, `source_route=None`).
    /// Filled later in the lifecycle once route_plan parsing is wired into
    /// the aggregator processors.
    pub source_route: Option<String>,
    /// Pool address when the venue is a known AMM. Optional; not all
    /// decoders surface it. Useful for downstream attribution.
    pub pool: Option<String>,

    // ── User attribution ───────────────────────────────────────────
    /// Every (user, group, engine) tuple that has this signer in their
    /// tracked-wallet list at decode time. One trigger event fans out to
    /// each — the coordinator emits a single `ObservedTrigger` and the
    /// writer publishes per-user.
    pub matched_users: Vec<UserMatch>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserMatch {
    pub user_id: String,
    pub group_id: String,
    /// `"attack"` or `"copytrade"`. Identifies which engine's config governs
    /// this user's trigger handling once the bridge stage lands.
    pub engine: &'static str,
    /// Optional human-readable label the user attached to this wallet.
    pub wallet_label: Option<String>,
}

impl ObservedTrigger {
    /// Stable composite key used by the coordinator's per-tx winner collapse.
    /// Same (sig, signer, family) → same trigger; coordinator picks one.
    pub fn collapse_key(&self) -> String {
        format!("{}|{}|{}", self.signature, self.signer, self.decoder_family)
    }
}
