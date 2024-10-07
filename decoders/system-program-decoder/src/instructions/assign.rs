use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01")]
pub struct Assign {
    pub onwer: solana_sdk::pubkey::Pubkey,
}

pub struct AssignAccounts {
    pub assigned_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Assign {
    type ArrangedAccounts = AssignAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let assigned_account = accounts.get(0)?;

        Some(AssignAccounts {
            assigned_account: *assigned_account,
        })
    }
}
