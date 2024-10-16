use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11179fd365b829f1")]
pub struct MigrateBinArray {}

pub struct MigrateBinArrayInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MigrateBinArray {
    type ArrangedAccounts = MigrateBinArrayInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;

        Some(MigrateBinArrayInstructionAccounts {
            lb_pair: lb_pair.pubkey,
        })
    }
}
