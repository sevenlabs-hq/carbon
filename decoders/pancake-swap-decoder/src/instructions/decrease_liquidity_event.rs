use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3ade563a44325538")]
pub struct DecreaseLiquidityEvent {
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub liquidity: u128,
    pub decrease_amount_0: u64,
    pub decrease_amount_1: u64,
    pub fee_amount_0: u64,
    pub fee_amount_1: u64,
    pub reward_amounts: [u64; 3],
    pub transfer_fee_0: u64,
    pub transfer_fee_1: u64,
}
