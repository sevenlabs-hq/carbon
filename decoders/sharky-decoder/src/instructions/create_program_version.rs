use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x67d800ee5c6bdb79")]
pub struct CreateProgramVersion {
    pub version: u8,
}

pub struct CreateProgramVersionInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub program_version: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateProgramVersion {
    type ArrangedAccounts = CreateProgramVersionInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let program_version = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let rent = accounts.get(3)?;

        Some(CreateProgramVersionInstructionAccounts {
            authority: authority.pubkey,
            program_version: program_version.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
