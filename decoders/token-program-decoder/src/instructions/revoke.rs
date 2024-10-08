use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct Revoke {}

pub struct RevokeAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl ArrangeAccounts for Revoke {
    type ArrangedAccounts = RevokeAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let owner = accounts.get(1)?;

        Some(RevokeAccounts {
            source: source.pubkey,
            owner: owner.pubkey,
            remaining_accounts: accounts.get(2..).unwrap_or_default().to_vec(),
        })
    }
}
