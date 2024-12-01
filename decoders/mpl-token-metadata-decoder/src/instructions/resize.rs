

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x38")]
pub struct Resize{
}

pub struct ResizeInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Resize {
    type ArrangedAccounts = ResizeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let edition = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let authority = accounts.get(4)?;
        let token = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(ResizeInstructionAccounts {
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            mint: mint.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            token: token.pubkey,
            system_program: system_program.pubkey,
        })
    }
}