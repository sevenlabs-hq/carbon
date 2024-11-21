use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd02fc29b116252ec")]
pub struct Collect {}

pub struct CollectInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pda_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Collect {
    type ArrangedAccounts = CollectInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let pda_account = accounts.get(1)?;

        Some(CollectInstructionAccounts {
            authority: authority.pubkey,
            pda_account: pda_account.pubkey,
        })
    }
}
