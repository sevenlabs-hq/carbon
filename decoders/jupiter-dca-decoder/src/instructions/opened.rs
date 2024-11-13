use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da6ac61094d4cbd6d")]
pub struct Opened {
    pub user_key: solana_sdk::pubkey::Pubkey,
    pub dca_key: solana_sdk::pubkey::Pubkey,
    pub in_deposited: u64,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub cycle_frequency: i64,
    pub in_amount_per_cycle: u64,
    pub created_at: i64,
}
