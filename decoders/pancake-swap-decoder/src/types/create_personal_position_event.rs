use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreatePersonalPositionEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub minter: solana_pubkey::Pubkey,
    pub nft_owner: solana_pubkey::Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub deposit_amount_0: u64,
    pub deposit_amount_1: u64,
    pub deposit_amount_0_transfer_fee: u64,
    pub deposit_amount_1_transfer_fee: u64,
}
