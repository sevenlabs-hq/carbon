use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d8604113fdd2db1ad")]
pub struct FilledEvent {
    pub user_key: solana_sdk::pubkey::Pubkey,
    pub dca_key: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub fee_mint: solana_sdk::pubkey::Pubkey,
    pub fee: u64,
}
