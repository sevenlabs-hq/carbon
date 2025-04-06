use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x43b2826d7e721c2a")]
pub struct MarginfiAccount {
    pub group: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub lending_account: LendingAccount,
    pub account_flags: u64,
    pub padding: [u64; 63],
}
