use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0A")]
pub struct AssignWithSeed {
    base: solana_sdk::pubkey::Pubkey,
    seed: u64,
    owner: solana_sdk::pubkey::Pubkey,
}

pub struct AssignWithSeedAccounts {
    pub assigned_account: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for AssignWithSeed {
    type ArrangedAccounts = AssignWithSeedAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let assigned_account = accounts.get(0)?;
        let base_account = accounts.get(1)?;

        Some(AssignWithSeedAccounts {
            assigned_account: *assigned_account,
            base_account: *base_account,
        })
    }
}
