use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d314f69d420221e54")]
pub struct IncreaseLiquidityEvent {
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub liquidity: u128,
    pub amount0: u64,
    pub amount1: u64,
    pub amount0_transfer_fee: u64,
    pub amount1_transfer_fee: u64,
}
