use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct IncreaseLiquidityEvent {
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub liquidity: u128,
    pub amount_0: u64,
    pub amount_1: u64,
    pub amount_0_transfer_fee: u64,
    pub amount_1_transfer_fee: u64,
}
