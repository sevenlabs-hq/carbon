use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct Allocate {
    space: u64,
}

pub struct AllocateAccounts {
    pub new_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Allocate {
    type ArrangedAccounts = AllocateAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let new_account = accounts.get(0)?;

        Some(AllocateAccounts {
            new_account: new_account.pubkey,
        })
    }
}
