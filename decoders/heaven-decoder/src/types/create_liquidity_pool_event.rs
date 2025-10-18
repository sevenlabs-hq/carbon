use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateLiquidityPoolEvent {
    pub liquidity_pool_id: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub base_token_input_transfer_fee_amount: u64,
    pub quote_token_input_transfer_fee_amount: u64,
    pub base_token_input_amount: u64,
    pub quote_token_input_amount: u64,
    pub lp_token_output_amount: u64,
    pub locked_lp: bool,
}
