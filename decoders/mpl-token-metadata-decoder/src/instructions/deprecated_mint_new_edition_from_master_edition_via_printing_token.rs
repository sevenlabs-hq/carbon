

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x03")]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingToken{
}

pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub master_token_account: solana_sdk::pubkey::Pubkey,
    pub edition_marker: solana_sdk::pubkey::Pubkey,
    pub burn_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub master_update_authority: solana_sdk::pubkey::Pubkey,
    pub master_metadata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub reservation_list: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {
    type ArrangedAccounts = DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let edition = accounts.get(1)?;
        let master_edition = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let mint_authority = accounts.get(4)?;
        let printing_mint = accounts.get(5)?;
        let master_token_account = accounts.get(6)?;
        let edition_marker = accounts.get(7)?;
        let burn_authority = accounts.get(8)?;
        let payer = accounts.get(9)?;
        let master_update_authority = accounts.get(10)?;
        let master_metadata = accounts.get(11)?;
        let token_program = accounts.get(12)?;
        let system_program = accounts.get(13)?;
        let rent = accounts.get(14)?;
        let reservation_list = accounts.get(15)?;

        Some(DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts {
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            master_edition: master_edition.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            printing_mint: printing_mint.pubkey,
            master_token_account: master_token_account.pubkey,
            edition_marker: edition_marker.pubkey,
            burn_authority: burn_authority.pubkey,
            payer: payer.pubkey,
            master_update_authority: master_update_authority.pubkey,
            master_metadata: master_metadata.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            reservation_list: reservation_list.pubkey,
        })
    }
}