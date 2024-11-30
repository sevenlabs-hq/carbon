

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x02")]
pub struct DeprecatedCreateMasterEdition{
}

pub struct DeprecatedCreateMasterEditionInstructionAccounts {
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub printing_mint_authority: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub one_time_printing_authorization_mint_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedCreateMasterEdition {
    type ArrangedAccounts = DeprecatedCreateMasterEditionInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let edition = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let printing_mint = accounts.get(2)?;
        let one_time_printing_authorization_mint = accounts.get(3)?;
        let update_authority = accounts.get(4)?;
        let printing_mint_authority = accounts.get(5)?;
        let mint_authority = accounts.get(6)?;
        let metadata = accounts.get(7)?;
        let payer = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let rent = accounts.get(11)?;
        let one_time_printing_authorization_mint_authority = accounts.get(12)?;

        Some(DeprecatedCreateMasterEditionInstructionAccounts {
            edition: edition.pubkey,
            mint: mint.pubkey,
            printing_mint: printing_mint.pubkey,
            one_time_printing_authorization_mint: one_time_printing_authorization_mint.pubkey,
            update_authority: update_authority.pubkey,
            printing_mint_authority: printing_mint_authority.pubkey,
            mint_authority: mint_authority.pubkey,
            metadata: metadata.pubkey,
            payer: payer.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            one_time_printing_authorization_mint_authority: one_time_printing_authorization_mint_authority.pubkey,
        })
    }
}