use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ReserveLiquidity {
    pub mint_pubkey: solana_pubkey::Pubkey,
    pub supply_vault: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub available_amount: u64,
    pub borrowed_amount_sf: u128,
    pub market_price_sf: u128,
    pub market_price_last_updated_ts: u64,
    pub mint_decimals: u64,
    pub deposit_limit_crossed_slot: u64,
    pub borrow_limit_crossed_slot: u64,
    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub accumulated_protocol_fees_sf: u128,
    pub accumulated_referrer_fees_sf: u128,
    pub pending_referrer_fees_sf: u128,
    pub absolute_referral_rate_sf: u128,
    pub token_program: solana_pubkey::Pubkey,
    #[serde(with = "BigArray")]
    pub padding2: [u64; 51],
    pub padding3: [u128; 32],
}
