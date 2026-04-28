//! `SurveillanceEvent` payload matching the existing TS interface in
//! `src/types/surveillance.ts:71-95`. Decimals come from the same atlas-ws
//! metadata Carbon hands us — no mint registry, no RPC fetch.

use serde::Serialize;
use solana_pubkey::Pubkey;
use solana_transaction_status::{TransactionStatusMeta, TransactionTokenBalance};

#[derive(Serialize, Clone, Debug)]
pub struct SurveillanceEventOut {
    #[serde(rename = "userId")]
    pub user_id: String, // for routing to Redis channel; not part of frontend payload
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "targetName")]
    pub target_name: String,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    #[serde(rename = "walletLabel")]
    pub wallet_label: String,
    pub signature: String,
    #[serde(rename = "eventType")]
    pub event_type: &'static str,
    #[serde(rename = "tokenAddress")]
    pub token_address: Option<String>,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: Option<String>,
    #[serde(rename = "solAmount")]
    pub sol_amount: f64,
    #[serde(rename = "tokenAmount")]
    pub token_amount: f64,
    #[serde(rename = "priceSol")]
    pub price_sol: f64,
    pub program: &'static str,
    pub counterparty: String,
    #[serde(rename = "blockTime")]
    pub block_time: String,
    pub slot: u64,
    /// Extra metadata stored only in Supabase `raw_data` column; not sent to
    /// frontend in the live event.
    #[serde(skip_serializing)]
    pub raw_data: serde_json::Value,
}

/// Walk `meta.post_token_balances` for the entry matching (owner, mint) and
/// return its `decimals`. None if the wallet didn't hold/touch this mint.
pub fn extract_decimals(
    meta: &TransactionStatusMeta,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Option<u8> {
    let owner_str = owner.to_string();
    let mint_str = mint.to_string();
    let post = meta.post_token_balances.as_ref()?;
    post.iter().find_map(|tb| {
        if tb.owner == owner_str && tb.mint == mint_str {
            Some(tb.ui_token_amount.decimals)
        } else {
            None
        }
    })
}

/// (post - pre) raw token balance delta for (owner, mint). Returns 0 when
/// either side is missing — caller decides whether to treat as no-op or fall
/// back to instruction args.
pub fn token_balance_delta_raw(
    meta: &TransactionStatusMeta,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Option<i128> {
    let owner_str = owner.to_string();
    let mint_str = mint.to_string();
    let pre = meta.pre_token_balances.as_ref()?;
    let post = meta.post_token_balances.as_ref()?;
    let find_amount = |tbs: &[TransactionTokenBalance]| -> Option<i128> {
        tbs.iter().find_map(|tb| {
            if tb.owner == owner_str && tb.mint == mint_str {
                tb.ui_token_amount.amount.parse::<i128>().ok()
            } else {
                None
            }
        })
    };
    let pre_amt = find_amount(pre).unwrap_or(0);
    let post_amt = find_amount(post).unwrap_or(0);
    Some(post_amt - pre_amt)
}

pub fn iso8601_block_time(t: Option<i64>) -> String {
    // atlas-ws sometimes hands us None for block_time even on confirmed
    // notifications; fall back to "received now" rather than emit an empty
    // string (which Postgres rejects as TIMESTAMPTZ). Drift vs actual
    // block_time is sub-second.
    t.and_then(|s| chrono::DateTime::<chrono::Utc>::from_timestamp(s, 0))
        .unwrap_or_else(chrono::Utc::now)
        .to_rfc3339()
}

pub fn fmt_decimal(raw: u64, decimals: u8) -> f64 {
    raw as f64 / 10f64.powi(decimals as i32)
}

/// Full account-keys list (static + ALT-loaded writable + ALT-loaded readonly),
/// stringified, in the order `account_index` indexes into. Used by
/// `owner_of_token_account` to map an SPL token account pubkey to its owner.
pub fn full_account_keys(
    meta: &TransactionStatusMeta,
    static_keys: &[Pubkey],
) -> Vec<String> {
    let mut keys: Vec<String> = static_keys.iter().map(|p| p.to_string()).collect();
    keys.extend(meta.loaded_addresses.writable.iter().map(|p| p.to_string()));
    keys.extend(meta.loaded_addresses.readonly.iter().map(|p| p.to_string()));
    keys
}

/// For a given SPL token account pubkey, look up its (owner_wallet, mint,
/// decimals) from `post_token_balances`. Falls back to `pre_token_balances`
/// if the account was closed by this tx and isn't in post.
pub fn owner_of_token_account(
    meta: &TransactionStatusMeta,
    full_keys: &[String],
    token_account: &Pubkey,
) -> Option<(String, String, u8)> {
    let target = token_account.to_string();
    let idx = full_keys.iter().position(|k| k == &target)?;
    let pick = |opt: &Option<Vec<TransactionTokenBalance>>| {
        opt.as_ref().and_then(|tbs| {
            tbs.iter().find_map(|tb| {
                if tb.account_index as usize == idx {
                    Some((tb.owner.clone(), tb.mint.clone(), tb.ui_token_amount.decimals))
                } else {
                    None
                }
            })
        })
    };
    pick(&meta.post_token_balances).or_else(|| pick(&meta.pre_token_balances))
}

pub fn safe_price_sol(sol_amount_lamports: u64, token_amount_raw: u64, decimals: u8) -> f64 {
    if token_amount_raw == 0 {
        return 0.0;
    }
    let sol = sol_amount_lamports as f64 / 1e9;
    let tokens = fmt_decimal(token_amount_raw, decimals);
    if tokens == 0.0 { 0.0 } else { sol / tokens }
}
