use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d641e57f9c4df9ace")]
pub struct CreatePersonalPositionEvent {
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub minter: solana_sdk::pubkey::Pubkey,
    pub nft_owner: solana_sdk::pubkey::Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub deposit_amount0: u64,
    pub deposit_amount1: u64,
    pub deposit_amount0_transfer_fee: u64,
    pub deposit_amount1_transfer_fee: u64,
}
