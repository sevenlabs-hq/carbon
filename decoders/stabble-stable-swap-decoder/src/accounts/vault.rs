use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xd308e82b02987577")]
pub struct Vault {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub withdraw_authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_authority_bump: u8,
    pub authority_bump: u8,
    pub is_active: bool,
    pub beneficiary: solana_sdk::pubkey::Pubkey,
    pub beneficiary_fee: u64,
    pub pending_admin: Option<solana_sdk::pubkey::Pubkey>,
}
