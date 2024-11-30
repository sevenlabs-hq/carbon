

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x39")]
pub struct CloseAccounts{
}

pub struct CloseAccountsInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseAccounts {
    type ArrangedAccounts = CloseAccountsInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let edition = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let destination = accounts.get(4)?;

        Some(CloseAccountsInstructionAccounts {
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
            destination: destination.pubkey,
        })
    }
}