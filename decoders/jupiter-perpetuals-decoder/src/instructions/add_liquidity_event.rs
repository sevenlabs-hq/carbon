use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1bb299ba2fc48c2d")]
pub struct AddLiquidityEvent {
    pub custody_key: solana_pubkey::Pubkey,
    pub pool_key: solana_pubkey::Pubkey,
    pub token_amount_in: u64,
    pub pre_pool_amount_usd: u128,
    pub token_amount_usd: u64,
    pub fee_bps: u64,
    pub token_amount_after_fee: u64,
    pub mint_amount_usd: u64,
    pub lp_amount: u64,
    pub post_pool_amount_usd: u128,
}
