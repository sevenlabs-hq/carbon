use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1ddfeae81afc69b47d")]
pub struct MigrateFeeAccountEvent {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub new_admin_token_a_fee: solana_sdk::pubkey::Pubkey,
    pub new_admin_token_b_fee: solana_sdk::pubkey::Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
