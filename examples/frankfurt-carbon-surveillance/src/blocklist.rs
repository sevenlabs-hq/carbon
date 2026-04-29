//! Surveillance-specific deny list for public infrastructure pubkeys —
//! DEX aggregators / router programs / router PDAs / MEV tip accounts /
//! tip payment programs. These addresses appear in *every* user's
//! routed-or-tipped transactions, so adding one to surveillance
//! produces noise events for the whole network's flow rather than the
//! watcher's own activity.
//!
//! This blocklist is **surveillance-specific**. Other Trailblaze
//! services (e.g. the trenches detector, copytrade, route latency
//! monitoring, snipe simulators) legitimately watch routers and tip
//! accounts and must not consult this list. The gate lives at the
//! surveillance HTTP add path, recover, and Realtime INSERT — three
//! layers so a direct DB insert can't slip through.

/// (address, human label). Addresses are exact base58 pubkeys.
pub const SURVEILLANCE_BLOCKLIST: &[(&str, &str)] = &[
    // ─── DEX aggregators / routers ──────────────────────────────────
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
    // ─── Jito (tip payment program + 8 tip accounts) ────────────────
    ("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt", "Jito tip payment program"),
    ("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5", "Jito tip account"),
    ("HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe", "Jito tip account"),
    ("Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY", "Jito tip account"),
    ("ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49", "Jito tip account"),
    ("DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh", "Jito tip account"),
    ("ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt", "Jito tip account"),
    ("3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT", "Jito tip account"),
    ("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiKaRUr", "Jito tip account"),
    // ─── Helius Sender (10 tip accounts; min 0.0002 SOL) ────────────
    ("4ACfpUFoaSD9bfPdeu6DBt89gB6ENTeHBXCAi87NhDEE", "Helius Sender tip account"),
    ("D2L6yPZ2FmmmTKPgzaMKdhu6EWZcTpLy1Vhx8uvZe7NZ", "Helius Sender tip account"),
    ("9bnz4RShgq1hAnLnZbP8kbgBg1kEmcJBYQq3gQbmnSta", "Helius Sender tip account"),
    ("5VY91ws6B2hMmBFRsXkoAAdsPHBJwRfBht4DXox3xkwn", "Helius Sender tip account"),
    ("2nyhqdwKcJZR2vcqCyrYsaPVdAnFoJjiksCXJ7hfEYgD", "Helius Sender tip account"),
    ("2q5pghRs6arqVjRvT5gfgWfWcHWmw1ZuCzphgd5KfWGJ", "Helius Sender tip account"),
    ("wyvPkWjVZz1M8fHQnMMCDTQDbkManefNNhweYk5WkcF", "Helius Sender tip account"),
    ("3KCKozbAaF75qEU33jtzozcJ29yJuaLJTy2jFdzUY8bT", "Helius Sender tip account"),
    ("4vieeGHPYPG2MmyPRcYjdiDmmhN3ww7hsFNap8pVN3Ey", "Helius Sender tip account"),
    ("4TQLFNWK8AovT1gFvda5jfw2oJeRMKEmw7aH6MGBJ3or", "Helius Sender tip account"),
    // ─── 0slot (12 tip accounts; min 0.001 SOL) ─────────────────────
    ("FCjUJZ1qozm1e8romw216qyfQMaaWKxWsuySnumVCCNe", "0slot tip account"),
    ("Cix2bHfqPcKcM233mzxbLk14kSggUUiz2A87fJtGivXr", "0slot tip account"),
    ("6fQaVhYZA4w3MBSXjJ81Vf6W1EDYeUPXpgVQ6UQyU1Av", "0slot tip account"),
    ("4HiwLEP2Bzqj3hM2ENxJuzhcPCdsafwiet3oGkMkuQY4", "0slot tip account"),
    ("7toBU3inhmrARGngC7z6SjyP85HgGMmCTEwGNRAcYnEK", "0slot tip account"),
    ("8mR3wB1nh4D6J9RUCugxUpc6ya8w38LPxZ3ZjcBhgzws", "0slot tip account"),
    ("6SiVU5WEwqfFapRuYCndomztEwDjvS5xgtEof3PLEGm9", "0slot tip account"),
    ("TpdxgNJBWZRL8UXF5mrEsyWxDWx9HQexA9P1eTWQ42p", "0slot tip account"),
    ("D8f3WkQu6dCF33cZxuAsrKHrGsqGP2yvAHf8mX6RXnwf", "0slot tip account"),
    ("GQPFicsy3P3NXxB5piJohoxACqTvWE9fKpLgdsMduoHE", "0slot tip account"),
    ("Ey2JEr8hDkgN8qKJGrLf2yFjRhW7rab99HVxwi5rcvJE", "0slot tip account"),
    ("4iUgjMT8q2hNZnLuhpqZ1QtiV8deFPy2ajvvjEpKKgsS", "0slot tip account"),
    ("3Rz8uD83QsU8wKvZbgWAPvCNDU6Fy8TSZTMcPm3RB6zt", "0slot tip account"),
    // ─── Nozomi / Temporal (17; rotated to avoid write-lock contention) ─
    ("TEMPaMeCRFAS9EKF53Jd6KpHxgL47uWLcpFArU1Fanq", "Nozomi/Temporal tip account"),
    ("noz3jAjPiHuBPqiSPkkugaJDkJscPuRhYnSpbi8UvC4", "Nozomi/Temporal tip account"),
    ("noz3str9KXfpKknefHji8L1mPgimezaiUyCHYMDv1GE", "Nozomi/Temporal tip account"),
    ("noz6uoYCDijhu1V7cutCpwxNiSovEwLdRHPwmgCGDNo", "Nozomi/Temporal tip account"),
    ("noz9EPNcT7WH6Sou3sr3GGjHQYVkN3DNirpbvDkv9YJ", "Nozomi/Temporal tip account"),
    ("nozc5yT15LazbLTFVZzoNZCwjh3yUtW86LoUyqsBu4L", "Nozomi/Temporal tip account"),
    ("nozFrhfnNGoyqwVuwPAW4aaGqempx4PU6g6D9CJMv7Z", "Nozomi/Temporal tip account"),
    ("nozievPk7HyK1Rqy1MPJwVQ7qQg2QoJGyP71oeDwbsu", "Nozomi/Temporal tip account"),
    ("noznbgwYnBLDHu8wcQVCEw6kDrXkPdKkydGJGNXGvL7", "Nozomi/Temporal tip account"),
    ("nozNVWs5N8mgzuD3qigrCG2UoKxZttxzZ85pvAQVrbP", "Nozomi/Temporal tip account"),
    ("nozpEGbwx4BcGp6pvEdAh1JoC2CQGZdU6HbNP1v2p6P", "Nozomi/Temporal tip account"),
    ("nozrhjhkCr3zXT3BiT4WCodYCUFeQvcdUkM7MqhKqge", "Nozomi/Temporal tip account"),
    ("nozrwQtWhEdrA6W8dkbt9gnUaMs52PdAv5byipnadq3", "Nozomi/Temporal tip account"),
    ("nozUacTVWub3cL4mJmGCYjKZTnE9RbdY5AP46iQgbPJ", "Nozomi/Temporal tip account"),
    ("nozWCyTPppJjRuw2fpzDhhWbW355fzosWSzrrMYB1Qk", "Nozomi/Temporal tip account"),
    ("nozWNju6dY353eMkMqURqwQEoM3SFgEKC6psLCSfUne", "Nozomi/Temporal tip account"),
    ("nozxNBgWohjR75vdspfxR5H9ceC7XXH99xpxhVGt3Bb", "Nozomi/Temporal tip account"),
    // ─── bloXroute (17 tip accounts) ────────────────────────────────
    ("3UQUKjhMKaY2S6bjcQD6yHB7utcZt5bfarRCmctpRtUd", "bloXroute tip account"),
    ("FogxVNs6Mm2w9rnGL1vkARSwJxvLE8mujTv3LK8RnUhF", "bloXroute tip account"),
    ("bLx7MvxGaKdKL7mEbpk9tC79z6MnBSJoJkuaEAPu6Nd", "bloXroute tip account"),
    ("bLx7XBqSg3LUPVf1bRgCnkJmgVZR8QEgDJBPqcRLHvp", "bloXroute tip account"),
    ("bLx8KeZxinPwy6kkUgyzMLeqb2ARNsWjADG1dhSsVba", "bloXroute tip account"),
    ("bLxADBknoNj8WAGw2W6GBYeq848Xx6ajhaymV1YvrHm", "bloXroute tip account"),
    ("bLxAc88vRBwvcUQJEgcxNfBLvHPikY4csNsUmPeWea2", "bloXroute tip account"),
    ("bLxQ88oCiTsL8Xj4YWekKi1hjrgmbE3J3FFZ2xZHR3h", "bloXroute tip account"),
    ("bLxS7NoLuynNRJ4mCnEE2YbtwJFttYsEyp2ME7rp2yt", "bloXroute tip account"),
    ("bLxW6mCov7VEbrKc3S9tcBRcfSzRnLCbNp3Dfn3SJG5", "bloXroute tip account"),
    ("bLxXSGXs4mYPTC5okZXed1qzvjNwNJ48QJ82hT2V7w7", "bloXroute tip account"),
    ("bLxYi3vojbbB7hVzVDVTdBLVPhp7GJ3ZB3BwdK5sFXi", "bloXroute tip account"),
    ("bLxhLPgBXtUpX4b1bH3HatuMGMSKT9GnwtuCGiMSAqe", "bloXroute tip account"),
    ("bLxpY1mniuFW4PgkNA4JiNxoeKHFszryi6tNgyZAiAA", "bloXroute tip account"),
    ("bLxuETxd2tgWxBALNwPzAfHhsik4BzD3nrEBCiPNZQD", "bloXroute tip account"),
    ("bLxuL2gK5FW7xfahvwLrxLyW76vcCpNsKQY2CmnE6kV", "bloXroute tip account"),
    ("bLxv4Hnub7nDJWHs8s17o9bGU65Bnx6Yqp2fqtMgHmm", "bloXroute tip account"),
    // ─── NextBlock (8 tip accounts) ─────────────────────────────────
    ("NEXTbLoCkB51HpLBLojQfpyVAMorm3zzKg7w9NFdqid", "NextBlock tip account"),
    ("nextBLoCkPMgmG8ZgJtABeScP35qLa2AMCNKntAP7Xc", "NextBlock tip account"),
    ("NextbLoCkVtMGcV47JzewQdvBpLqT9TxQFozQkN98pE", "NextBlock tip account"),
    ("NexTbLoCkWykbLuB1NkjXgFWkX9oAtcoagQegygXXA2", "NextBlock tip account"),
    ("NeXTBLoCKs9F1y5PJS9CKrFNNLU1keHW71rfh7KgA1X", "NextBlock tip account"),
    ("NexTBLockJYZ7QD7p2byrUa6df8ndV2WSd8GkbWqfbb", "NextBlock tip account"),
    ("neXtBLock1LeC67jYd1QdAa32kbVeubsfPNTJC1V5At", "NextBlock tip account"),
    ("nEXTBLockYgngeRmRrjDV31mGSekVPqZoMGhQEZtPVG", "NextBlock tip account"),
    // ─── Astralane (16 tip accounts) ────────────────────────────────
    ("ASTRaoF93eYt73TYvwtsv6fMWHWbGmMUZfVZPo3CRU9C", "Astralane tip account"),
    ("AStRAnpi6kFrKypragExgeRoJ1QnKH7pbSjLAKQVWUum", "Astralane tip account"),
    ("Astran35aiQUF57XZsmkWMtNCtXGLzs8upfiqXxth2bz", "Astralane tip account"),
    ("AStrAJv2RN2hKCHxwUMtqmSxgdcNZbihCwc1mCSnG83W", "Astralane tip account"),
    ("AsTRAEoyMofR3vUPpf9k68Gsfb6ymTZttEtsAbv8Bk4d", "Astralane tip account"),
    ("AsTRADtvb6tTmrsqULQ9Wji9PigDMjhfEMza6zkynEvV", "Astralane tip account"),
    ("AstrABAu8CBTyuPXpV4eSCJ5fePEPnxN8NqBaPKQ9fHR", "Astralane tip account"),
    ("AsTra79FET4aCKWspPqeSFvjJNyp96SvAnrmyAxqg5b7", "Astralane tip account"),
    ("AstrA1ejL4UeXC2SBP4cpeEmtcFPZVLxx3XGKXyCW6to", "Astralane tip account"),
    ("astrazznxsGUhWShqgNtAdfrzP2G83DzcWVJDxwV9bF", "Astralane tip account"),
    ("astra4uejePWneqNaJKuFFA8oonqCE1sqF6b45kDMZm", "Astralane tip account"),
    ("astra9xWY93QyfG6yM8zwsKsRodscjQ2uU2HKNL5prk", "Astralane tip account"),
    ("astraRVUuTHjpwEVvNBeQEgwYx9w9CFyfxjYoobCZhL", "Astralane tip account"),
    ("astraEJ2fEj8Xmy6KLG7B3VfbKfsHXhHrNdCQx7iGJK", "Astralane tip account"),
    ("astraubkDw81n4LuutzSQ8uzHCv4BhPVhfvTcYv8SKC", "Astralane tip account"),
    ("astraZW5GLFefxNPAatceHhYjfA1ciq9gvfEg2S47xk", "Astralane tip account"),
    ("astrawVNP4xDBKT7rAdxrLYiTSTdqtUr63fSMduivXK", "Astralane tip account"),
];

/// Returns the human-readable label if `address` is on the surveillance
/// blocklist, else None. Used to (a) decide whether to reject the add
/// and (b) include a useful error message back to the caller.
///
/// Function name kept as `router_label` for API stability across the
/// codebase; the underlying list now also covers tip accounts.
pub fn router_label(address: &str) -> Option<&'static str> {
    SURVEILLANCE_BLOCKLIST
        .iter()
        .find(|(a, _)| *a == address)
        .map(|(_, label)| *label)
}

/// Convenience for the rejection reason string surfaced to API callers.
pub fn rejection_reason(address: &str) -> Option<String> {
    router_label(address).map(|label| {
        format!(
            "wallet is public infrastructure ({}); it can't be added to surveillance",
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
    fn tip_accounts_are_blocked() {
        // The originally-reported case: a 0slot tip account that got
        // mass-imported as a "trader" wallet. Pin it here so a
        // regression on the blocklist would fail loudly.
        assert_eq!(
            router_label("FCjUJZ1qozm1e8romw216qyfQMaaWKxWsuySnumVCCNe"),
            Some("0slot tip account")
        );
        // One from each provider — sanity check the block covers the
        // full list, not just one service.
        assert_eq!(
            router_label("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"),
            Some("Jito tip account")
        );
        assert_eq!(
            router_label("4ACfpUFoaSD9bfPdeu6DBt89gB6ENTeHBXCAi87NhDEE"),
            Some("Helius Sender tip account")
        );
        assert_eq!(
            router_label("TEMPaMeCRFAS9EKF53Jd6KpHxgL47uWLcpFArU1Fanq"),
            Some("Nozomi/Temporal tip account")
        );
        assert_eq!(
            router_label("3UQUKjhMKaY2S6bjcQD6yHB7utcZt5bfarRCmctpRtUd"),
            Some("bloXroute tip account")
        );
        assert_eq!(
            router_label("NEXTbLoCkB51HpLBLojQfpyVAMorm3zzKg7w9NFdqid"),
            Some("NextBlock tip account")
        );
        assert_eq!(
            router_label("ASTRaoF93eYt73TYvwtsv6fMWHWbGmMUZfVZPo3CRU9C"),
            Some("Astralane tip account")
        );
        // Jito tip payment program (not a tip account but tip-adjacent
        // public infrastructure)
        assert_eq!(
            router_label("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt"),
            Some("Jito tip payment program")
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
        assert!(reason.contains("can't be added to surveillance"));
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
            SURVEILLANCE_BLOCKLIST.iter().map(|(a, _)| *a).collect();
        assert_eq!(addrs.len(), SURVEILLANCE_BLOCKLIST.len());
    }
}
