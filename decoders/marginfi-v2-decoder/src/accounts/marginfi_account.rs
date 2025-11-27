use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x43b2826d7e721c2a")]
pub struct MarginfiAccount {
    pub group: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub lending_account: LendingAccount,
    pub account_flags: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u64; 63],
}
