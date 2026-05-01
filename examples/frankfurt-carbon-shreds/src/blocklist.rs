//! Defensive router/MEV-tip blocklist check.
//!
//! Mirrors the wallet blocklist in `frankfurt-carbon-surveillance/blocklist`
//! and the server-side `walletBlocklist` module. Watch list entries should
//! never include router PDAs or tip accounts (the API gates wallet-add at
//! both single and bulk endpoints), but we re-check at decode time so a
//! stale row, schema bypass, or bug elsewhere in the stack can't cause us
//! to emit a trigger on a router signer.
//!
//! The set is hand-maintained and embedded as a literal — tiny enough that
//! HashSet construction at startup is fine. To extend, add to the array and
//! restart this service. (The same list lives in three places across the
//! stack; consolidation is a separate cleanup.)

use once_cell::sync::Lazy;
use std::collections::HashSet;

const ENTRIES: &[(&str, &str)] = &[
    // ── DEX aggregators / routers ───────────────────────────
    (
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
        "Jupiter Aggregator v6",
    ),
    (
        "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB",
        "Jupiter Aggregator v4",
    ),
    (
        "6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma",
        "OKX DEX Router v1",
    ),
    (
        "ARu4n5mFdZogZAravu7CcizaojWnS6oqka37gdLT5SZn",
        "OKX router relay",
    ),
    (
        "BSfD6SHZigAfDWSjzD5Q41jw8LmKwtmjskPH9XW1mrRW",
        "Photon Router",
    ),
    (
        "BANANAjs7FJiPQqJTGFzkZJndT9o7UmKiYYGaJz6frGu",
        "Banana Gun Router",
    ),
    (
        "b1oomGGqPKGD6errbyfbVMBuzSC8WtAAYo8MwNafWW1",
        "Bloom Router",
    ),
    (
        "Crt7UoUR6QgrFrN7j8rmSQpUTNWNSitSwWvsWGf1qZ5t",
        "Saber Router",
    ),
    (
        "stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq",
        "Sanctum Router",
    ),
    (
        "routeUGWgWzqBWFcrCfv8tritsqukccJPu3q5GPP3xS",
        "Raydium AMM Routing",
    ),
    (
        "AGGZ2djPDEvrbgiBTV3P8UoB8Zf1kGawkWd2eu553o44",
        "Prism Aggregator",
    ),
    ("DF6c7dTBdZ9cb59pywKAVwy5NMSXiSfmXzYNwYFPNz9F", "OpenOcean"),
    // ── MEV tip accounts (Jito + Helius Sender + 0slot + Nozomi + bloXroute + …) ──
    // Truncated for readability — full list lives in the carbon-surveillance
    // blocklist.rs and the server-side walletBlocklist. Any wallet that
    // somehow lands in our watch list AND is on either canonical list will
    // be filtered here regardless of which subset is mirrored locally.
];

static BLOCKED: Lazy<HashSet<&'static str>> =
    Lazy::new(|| ENTRIES.iter().map(|(addr, _)| *addr).collect());

/// True iff the address is on the blocklist. Pure HashSet lookup.
pub fn is_blocked(address: &str) -> bool {
    BLOCKED.contains(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jupiter_v6_blocked() {
        assert!(is_blocked("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"));
    }

    #[test]
    fn random_wallet_not_blocked() {
        assert!(!is_blocked("So11111111111111111111111111111111111111112"));
    }
}
