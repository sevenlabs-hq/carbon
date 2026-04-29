//! Surveillance-specific deny list for public DEX aggregators / router
//! programs / router PDAs. These addresses appear in *every* user's
//! routed transactions, so adding one to surveillance produces noise
//! events for the whole network of routed swaps — none of which
//! represent the watcher's own activity.
//!
//! This blocklist is **surveillance-specific**. Other Trailblaze
//! services (e.g. the trenches detector, copytrade, route latency
//! monitoring) legitimately watch routers, and must not consult this
//! list. The gate lives at the surveillance HTTP add path, recover, and
//! Realtime INSERT — three layers so a direct DB insert can't slip
//! through.

/// (address, human label). Addresses are exact base58 pubkeys.
pub const ROUTER_BLOCKLIST: &[(&str, &str)] = &[
    ("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4", "Jupiter Aggregator v6"),
    ("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB", "Jupiter Aggregator v4"),
    ("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma", "OKX DEX Router v1"),
    ("ARu4n5mFdZogZAravu7CcizaojWnS6oqka37gdLT5SZn", "OKX router relay"),
    ("BSfD6SHZigAfDWSjzD5Q41jw8LmKwtmjskPH9XW1mrRW", "Photon Router"),
    ("BANANAjs7FJiPQqJTGFzkZJndT9o7UmKiYYGaJz6frGu", "Banana Gun Router"),
    ("b1oomGGqPKGD6errbyfbVMBuzSC8WtAAYo8MwNafWW1", "Bloom Router"),
    ("Crt7UoUR6QgrFrN7j8rmSQpUTNWNSitSwWvsWGf1qZ5t", "Saber Router"),
    ("stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq", "Sanctum Router"),
    ("routeUGWgWzqBWFcrCfv8tritsqukccJPu3q5GPP3xS", "Raydium AMM Routing"),
    ("AGGZ2djPDEvrbgiBTV3P8UoB8Zf1kGawkWd2eu553o44", "Prism Aggregator"),
    ("DF6c7dTBdZ9cb59pywKAVwy5NMSXiSfmXzYNwYFPNz9F", "OpenOcean"),
];

/// Returns the human-readable label if `address` is on the surveillance
/// blocklist, else None. Used to (a) decide whether to reject the add
/// and (b) include a useful error message back to the caller.
pub fn router_label(address: &str) -> Option<&'static str> {
    ROUTER_BLOCKLIST
        .iter()
        .find(|(a, _)| *a == address)
        .map(|(_, label)| *label)
}

/// Convenience for the rejection reason string surfaced to API callers.
pub fn rejection_reason(address: &str) -> Option<String> {
    router_label(address).map(|label| {
        format!(
            "wallet is a known router ({}); routers can't be added to surveillance",
            label
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_routers_are_blocked() {
        assert_eq!(
            router_label("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
            Some("Jupiter Aggregator v6")
        );
        assert_eq!(
            router_label("ARu4n5mFdZogZAravu7CcizaojWnS6oqka37gdLT5SZn"),
            Some("OKX router relay")
        );
        assert_eq!(
            router_label("DF6c7dTBdZ9cb59pywKAVwy5NMSXiSfmXzYNwYFPNz9F"),
            Some("OpenOcean")
        );
    }

    #[test]
    fn regular_wallets_are_not_blocked() {
        // Real watched wallets observed in production
        assert_eq!(
            router_label("3PpgE1PkUoftrPkZ5TcYzqaVHJs27baJsd4WathmNkuP"),
            None
        );
        assert_eq!(
            router_label("BeAVrLPgkFF7V6TQ1CFN17eqeCsbhTz8PVMbmuahharp"),
            None
        );
        assert_eq!(router_label(""), None);
        assert_eq!(router_label("not_a_pubkey"), None);
    }

    #[test]
    fn rejection_reason_includes_label() {
        let reason = rejection_reason("ARu4n5mFdZogZAravu7CcizaojWnS6oqka37gdLT5SZn").unwrap();
        assert!(reason.contains("OKX router relay"));
        assert!(reason.contains("routers can't be added"));
    }

    #[test]
    fn rejection_reason_none_for_regular_wallet() {
        assert_eq!(
            rejection_reason("3PpgE1PkUoftrPkZ5TcYzqaVHJs27baJsd4WathmNkuP"),
            None
        );
    }

    #[test]
    fn blocklist_has_no_duplicates() {
        let addrs: std::collections::HashSet<_> =
            ROUTER_BLOCKLIST.iter().map(|(a, _)| *a).collect();
        assert_eq!(addrs.len(), ROUTER_BLOCKLIST.len());
    }
}
