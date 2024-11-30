
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0b")]
pub struct MintNewEditionFromMasterEditionViaToken{
    pub mint_new_edition_from_master_edition_via_token_args: MintNewEditionFromMasterEditionViaTokenArgs,
}

pub struct MintNewEditionFromMasterEditionViaTokenInstructionAccounts {
    pub new_metadata: solana_sdk::pubkey::Pubkey,
    pub new_edition: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub new_mint: solana_sdk::pubkey::Pubkey,
    pub edition_mark_pda: solana_sdk::pubkey::Pubkey,
    pub new_mint_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_account_owner: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub new_metadata_update_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintNewEditionFromMasterEditionViaToken {
    type ArrangedAccounts = MintNewEditionFromMasterEditionViaTokenInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let new_metadata = accounts.get(0)?;
        let new_edition = accounts.get(1)?;
        let master_edition = accounts.get(2)?;
        let new_mint = accounts.get(3)?;
        let edition_mark_pda = accounts.get(4)?;
        let new_mint_authority = accounts.get(5)?;
        let payer = accounts.get(6)?;
        let token_account_owner = accounts.get(7)?;
        let token_account = accounts.get(8)?;
        let new_metadata_update_authority = accounts.get(9)?;
        let metadata = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let system_program = accounts.get(12)?;
        let rent = accounts.get(13)?;

        Some(MintNewEditionFromMasterEditionViaTokenInstructionAccounts {
            new_metadata: new_metadata.pubkey,
            new_edition: new_edition.pubkey,
            master_edition: master_edition.pubkey,
            new_mint: new_mint.pubkey,
            edition_mark_pda: edition_mark_pda.pubkey,
            new_mint_authority: new_mint_authority.pubkey,
            payer: payer.pubkey,
            token_account_owner: token_account_owner.pubkey,
            token_account: token_account.pubkey,
            new_metadata_update_authority: new_metadata_update_authority.pubkey,
            metadata: metadata.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}