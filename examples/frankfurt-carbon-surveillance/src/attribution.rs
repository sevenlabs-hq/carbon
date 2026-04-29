//! Pure attribution helpers — given a decoded ix's accounts (or a typed
//! event payload), return which pubkey is the *user* / *trader* for the
//! purposes of surveillance attribution. The processors call these and
//! then look the returned pubkey up in `state::WATCH`.
//!
//! These exist as standalone functions so the slot-index decisions can be
//! pinned by unit tests — getting `accounts[1]` vs `accounts[0]` wrong on
//! PumpSwap or scanning the wrong subset in AggWatch silently breaks
//! aggregator-routed coverage. The `processors_test` module below
//! exercises every shape that matters.

use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

/// PumpSwap's `Buy`/`Sell`/`BuyExactQuoteIn` ixs put the user at
/// `accounts[1]` (per the IDL: pool=0, user=1, global_config=2,
/// base_mint=3, …). Fee payer is whoever signed the outer tx — could be
/// a router signing on the user's behalf.
pub fn pumpswap_user(accounts: &[AccountMeta]) -> Option<Pubkey> {
    accounts.get(1).map(|a| a.pubkey)
}

/// PumpSwap's base mint is `accounts[3]`. Used for token-balance lookups.
pub fn pumpswap_base_mint(accounts: &[AccountMeta]) -> Option<Pubkey> {
    accounts.get(3).map(|a| a.pubkey)
}

/// Generic AggWatch attribution: every pubkey appearing in the decoded
/// ix's `accounts` vec that's in the watch list, deduped (a single ix
/// can list the same account twice, and we want one candidate per
/// matched wallet at most). Returned in first-seen order so callers
/// observe a stable iteration when running tests.
///
/// `is_watched` is the predicate used to decide membership — the
/// processor passes a closure over `state::lookup`, tests pass a closure
/// over a hand-built set so they don't need to mutate global WATCH.
pub fn aggwatch_matched_wallets<F: Fn(&str) -> bool>(
    accounts: &[AccountMeta],
    is_watched: F,
) -> Vec<Pubkey> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for a in accounts {
        let s = a.pubkey.to_string();
        if !seen.insert(s.clone()) {
            continue;
        }
        if is_watched(&s) {
            out.push(a.pubkey);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pk(byte: u8) -> Pubkey {
        let mut bytes = [0u8; 32];
        bytes[0] = byte;
        Pubkey::new_from_array(bytes)
    }

    fn am(p: Pubkey) -> AccountMeta {
        AccountMeta::new(p, false)
    }

    #[test]
    fn pumpswap_user_is_index_1() {
        // pool, user, global_config, base_mint, …
        let accounts = vec![am(pk(0xA0)), am(pk(0xB1)), am(pk(0xC2)), am(pk(0xD3))];
        assert_eq!(pumpswap_user(&accounts), Some(pk(0xB1)));
    }

    #[test]
    fn pumpswap_base_mint_is_index_3() {
        let accounts = vec![am(pk(0xA0)), am(pk(0xB1)), am(pk(0xC2)), am(pk(0xD3))];
        assert_eq!(pumpswap_base_mint(&accounts), Some(pk(0xD3)));
    }

    #[test]
    fn pumpswap_user_returns_none_on_truncated_accounts() {
        let accounts = vec![am(pk(0xA0))]; // only pool, no user
        assert_eq!(pumpswap_user(&accounts), None);
    }

    #[test]
    fn aggwatch_finds_watched_anywhere_in_accounts() {
        let watched = pk(0x42);
        let watch = std::collections::HashSet::from([watched.to_string()]);
        // watched wallet sits at the 5th slot — past where signer/fee_payer
        // would have been, exactly the case our old fee_payer-based code
        // used to drop on the floor.
        let accounts = vec![
            am(pk(0xAA)), // pool
            am(pk(0xBB)), // some PDA
            am(pk(0xCC)), // global config
            am(pk(0xDD)), // mint
            am(watched),  // user (counterparty, NOT signer)
            am(pk(0xFF)),
        ];
        let matches = aggwatch_matched_wallets(&accounts, |s| watch.contains(s));
        assert_eq!(matches, vec![watched]);
    }

    #[test]
    fn aggwatch_emits_no_match_when_signer_is_not_watched() {
        // The whole point of the regression: signer = unwatched router,
        // ix accounts contain only unwatched pubkeys → no candidate.
        let watch = std::collections::HashSet::from([pk(0x42).to_string()]);
        let accounts = vec![am(pk(0xAA)), am(pk(0xBB)), am(pk(0xCC))];
        let matches = aggwatch_matched_wallets(&accounts, |s| watch.contains(s));
        assert!(matches.is_empty());
    }

    #[test]
    fn aggwatch_dedupes_repeated_account() {
        // Some ixs list the same authority twice (e.g. authority + fee
        // payer slots both filled by the user). One match expected.
        let watched = pk(0x42);
        let watch = std::collections::HashSet::from([watched.to_string()]);
        let accounts = vec![am(watched), am(pk(0xBB)), am(watched), am(pk(0xCC))];
        let matches = aggwatch_matched_wallets(&accounts, |s| watch.contains(s));
        assert_eq!(matches, vec![watched]);
    }

    #[test]
    fn aggwatch_emits_per_distinct_match_in_order() {
        // Multi-watched: a copytrade tx where two of OUR users are both
        // counterparties. Each gets its own candidate, in first-seen order.
        let a = pk(0xAA);
        let b = pk(0xBB);
        let watch =
            std::collections::HashSet::from([a.to_string(), b.to_string()]);
        let accounts = vec![am(pk(0x01)), am(b), am(pk(0x02)), am(a), am(pk(0x03))];
        let matches = aggwatch_matched_wallets(&accounts, |s| watch.contains(s));
        // First-seen order: b appeared at index 1, a at index 3
        assert_eq!(matches, vec![b, a]);
    }

    #[test]
    fn aggwatch_does_not_match_unwatched_pubkeys_even_with_long_account_list() {
        let watch = std::collections::HashSet::from([pk(0x42).to_string()]);
        // 50-element accounts list, none watched → empty.
        let accounts: Vec<_> = (0u8..50).map(|i| am(pk(i.wrapping_add(1)))).collect();
        let matches = aggwatch_matched_wallets(&accounts, |s| watch.contains(s));
        assert!(matches.is_empty());
    }
}
