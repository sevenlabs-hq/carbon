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
use solana_transaction_status::TransactionStatusMeta;

/// Per frankfurt-node's walletTransferDecode: 1000 lamports = 0.000001 SOL.
/// Below this we consider the SOL change rent-noise / floor-rounding.
pub const SOL_DUST_LAMPORTS: i128 = 1_000;
/// Smallest non-zero raw token amount.
pub const TOKEN_DUST_RAW: i128 = 1;

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

/// True if the watched wallet has a meaningful net SOL or token-balance
/// delta in the tx (meaningful = above dust). Used by AggWatch to
/// suppress ProgramActivity rows for routing-intermediary wallets — a
/// wallet that signs no ix but appears in many account slots as a proxy
/// has net-zero balance change (every in is matched by an out). Only
/// the actual user/trader has a real delta. Mirrors frankfurt-node's
/// walletTransferDecode net-balance heuristic.
///
/// Fee is added back for the fee_payer so the SOL delta reflects only
/// transfers, not the network charge.
pub fn watched_has_meaningful_delta(
    meta: &TransactionStatusMeta,
    static_keys: &[Pubkey],
    fee_payer: &Pubkey,
    wallet: &Pubkey,
) -> bool {
    // SOL delta
    if let Some(idx) = static_keys.iter().position(|k| k == wallet) {
        let pre = meta.pre_balances.get(idx).copied().unwrap_or(0) as i128;
        let post = meta.post_balances.get(idx).copied().unwrap_or(0) as i128;
        let mut delta = post - pre;
        if wallet == fee_payer {
            delta += meta.fee as i128;
        }
        if delta.unsigned_abs() as i128 >= SOL_DUST_LAMPORTS {
            return true;
        }
    }
    // Token-balance delta per (wallet, mint). post - pre across each
    // unique mint the wallet appears in. Multiple accounts of the same
    // mint owned by the same wallet are summed (Wallet has multiple
    // ATAs for the same mint case).
    let wallet_str = wallet.to_string();
    let mut by_mint_pre: std::collections::HashMap<String, i128> =
        std::collections::HashMap::new();
    let mut by_mint_post: std::collections::HashMap<String, i128> =
        std::collections::HashMap::new();
    if let Some(pre) = meta.pre_token_balances.as_ref() {
        for tb in pre {
            if tb.owner == wallet_str {
                let amt: i128 = tb.ui_token_amount.amount.parse().unwrap_or(0);
                *by_mint_pre.entry(tb.mint.clone()).or_default() += amt;
            }
        }
    }
    if let Some(post) = meta.post_token_balances.as_ref() {
        for tb in post {
            if tb.owner == wallet_str {
                let amt: i128 = tb.ui_token_amount.amount.parse().unwrap_or(0);
                *by_mint_post.entry(tb.mint.clone()).or_default() += amt;
            }
        }
    }
    for mint in by_mint_pre.keys().chain(by_mint_post.keys()) {
        let pre = by_mint_pre.get(mint).copied().unwrap_or(0);
        let post = by_mint_post.get(mint).copied().unwrap_or(0);
        let delta = post - pre;
        if delta.unsigned_abs() as i128 >= TOKEN_DUST_RAW {
            return true;
        }
    }
    false
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
    use solana_transaction_status::{TransactionTokenBalance};

    fn pk(byte: u8) -> Pubkey {
        let mut bytes = [0u8; 32];
        bytes[0] = byte;
        Pubkey::new_from_array(bytes)
    }

    fn am(p: Pubkey) -> AccountMeta {
        AccountMeta::new(p, false)
    }

    fn token_balance(
        account_index: u8,
        owner: &Pubkey,
        mint: &str,
        raw: &str,
        decimals: u8,
    ) -> TransactionTokenBalance {
        TransactionTokenBalance {
            account_index,
            mint: mint.into(),
            ui_token_amount: solana_account_decoder_client_types::token::UiTokenAmount {
                amount: raw.into(),
                decimals,
                ui_amount: None,
                ui_amount_string: raw.into(),
            },
            owner: owner.to_string(),
            program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".into(),
        }
    }

    fn empty_meta() -> TransactionStatusMeta {
        TransactionStatusMeta::default()
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

    #[test]
    fn delta_meaningful_when_sol_balance_moved() {
        let wallet = pk(0xA1);
        let static_keys = vec![pk(0x01), wallet];
        let mut meta = empty_meta();
        meta.pre_balances = vec![1_000_000, 5_000_000_000];
        meta.post_balances = vec![1_000_000, 4_500_000_000]; // wallet sent 0.5 SOL
        meta.fee = 5000;
        // wallet is NOT fee_payer (idx 0 is fee_payer), so no fee adjustment
        assert!(watched_has_meaningful_delta(&meta, &static_keys, &pk(0x01), &wallet));
    }

    #[test]
    fn delta_meaningful_when_fee_payer_after_fee_backout() {
        // Wallet is fee_payer. post-pre = -9000. After fee backout
        // (+5000), residual = -4000. abs(4000) >= 1000 dust → meaningful.
        let wallet = pk(0xA1);
        let static_keys = vec![wallet];
        let mut meta = empty_meta();
        meta.pre_balances = vec![1_000_000];
        meta.post_balances = vec![991_000];
        meta.fee = 5000;
        assert!(watched_has_meaningful_delta(&meta, &static_keys, &wallet, &wallet));
    }

    #[test]
    fn delta_below_dust_for_fee_payer_is_not_meaningful() {
        // Wallet is fee_payer. post-pre = -5500. After fee backout
        // (+5000), residual = -500. abs(500) < 1000 → NOT meaningful
        // (typical for a dust SOL change inside a no-op tx).
        let wallet = pk(0xA1);
        let static_keys = vec![wallet];
        let mut meta = empty_meta();
        meta.pre_balances = vec![1_000_000];
        meta.post_balances = vec![994_500];
        meta.fee = 5000;
        assert!(!watched_has_meaningful_delta(&meta, &static_keys, &wallet, &wallet));
    }

    #[test]
    fn delta_not_meaningful_when_only_fee_charged() {
        let wallet = pk(0xA1);
        let static_keys = vec![wallet];
        let mut meta = empty_meta();
        meta.pre_balances = vec![1_000_000];
        meta.post_balances = vec![995_000]; // exact -5000 (just the fee)
        meta.fee = 5000;
        assert!(!watched_has_meaningful_delta(&meta, &static_keys, &wallet, &wallet));
    }

    #[test]
    fn delta_meaningful_when_token_balance_moved() {
        let wallet = pk(0xA1);
        let static_keys = vec![wallet];
        let mut meta = empty_meta();
        meta.pre_balances = vec![1_000_000];
        meta.post_balances = vec![1_000_000];
        let mint = "5tDxd4G8DkfDnFmqvJpaTtcjJq4Hc1XSshM6QGHHMshv";
        meta.pre_token_balances = Some(vec![token_balance(0, &wallet, mint, "1000000", 6)]);
        meta.post_token_balances = Some(vec![token_balance(0, &wallet, mint, "2000000", 6)]); // +1M raw
        assert!(watched_has_meaningful_delta(&meta, &static_keys, &wallet, &wallet));
    }

    #[test]
    fn delta_not_meaningful_for_routing_intermediary() {
        // ARu4n5mF-style: every token "in" matched by a token "out".
        // Wallet appears in pre/post for the same mint at the same balance.
        let wallet = pk(0xA1);
        let static_keys = vec![pk(0x01), wallet]; // wallet is NOT fee_payer
        let mut meta = empty_meta();
        meta.pre_balances = vec![1_000_000, 5_000_000_000];
        meta.post_balances = vec![1_000_000, 5_000_000_000]; // SOL: zero delta
        meta.fee = 5000;
        let mint = "5tDxd4G8DkfDnFmqvJpaTtcjJq4Hc1XSshM6QGHHMshv";
        meta.pre_token_balances = Some(vec![token_balance(0, &wallet, mint, "1000000", 6)]);
        meta.post_token_balances = Some(vec![token_balance(0, &wallet, mint, "1000000", 6)]); // unchanged
        assert!(!watched_has_meaningful_delta(&meta, &static_keys, &pk(0x01), &wallet));
    }

    #[test]
    fn delta_not_meaningful_when_wallet_not_in_tx_at_all() {
        let wallet = pk(0xA1);
        let static_keys = vec![pk(0x01), pk(0x02)]; // wallet absent
        let meta = empty_meta();
        assert!(!watched_has_meaningful_delta(&meta, &static_keys, &pk(0x01), &wallet));
    }
}
