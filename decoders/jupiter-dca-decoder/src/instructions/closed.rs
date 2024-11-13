use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d321f579b87dcc3ef")]
pub struct Closed {
    pub user_key: solana_sdk::pubkey::Pubkey,
    pub dca_key: solana_sdk::pubkey::Pubkey,
    pub in_deposited: u64,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub cycle_frequency: i64,
    pub in_amount_per_cycle: u64,
    pub created_at: i64,
    pub total_in_withdrawn: u64,
    pub total_out_withdrawn: u64,
    pub unfilled_amount: u64,
    pub user_closed: bool,
}
