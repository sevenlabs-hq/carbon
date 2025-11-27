use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d318e48a6e61d5454")]
pub struct CreateOrderEvent {
    pub order_key: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
    pub output_token_program: solana_pubkey::Pubkey,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expired_at: Option<i64>,
    pub fee_bps: u16,
    pub fee_account: solana_pubkey::Pubkey,
}
