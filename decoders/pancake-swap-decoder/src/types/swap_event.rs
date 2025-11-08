use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_account_0: solana_pubkey::Pubkey,
    pub token_account_1: solana_pubkey::Pubkey,
    pub amount_0: u64,
    pub transfer_fee_0: u64,
    pub amount_1: u64,
    pub transfer_fee_1: u64,
    pub zero_for_one: bool,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick: i32,
}
