

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x09")]
pub struct DeprecatedMintPrintingTokens{
}

pub struct DeprecatedMintPrintingTokensInstructionAccounts {
    pub destination: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedMintPrintingTokens {
    type ArrangedAccounts = DeprecatedMintPrintingTokensInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let destination = accounts.get(0)?;
        let printing_mint = accounts.get(1)?;
        let update_authority = accounts.get(2)?;
        let metadata = accounts.get(3)?;
        let master_edition = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let rent = accounts.get(6)?;

        Some(DeprecatedMintPrintingTokensInstructionAccounts {
            destination: destination.pubkey,
            printing_mint: printing_mint.pubkey,
            update_authority: update_authority.pubkey,
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}