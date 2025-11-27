use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b")]
pub struct MintNewEditionFromMasterEditionViaToken {
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MintNewEditionFromMasterEditionViaTokenInstructionAccounts {
    pub new_metadata: solana_pubkey::Pubkey,
    pub new_edition: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub new_mint: solana_pubkey::Pubkey,
    pub edition_mark_pda: solana_pubkey::Pubkey,
    pub new_mint_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_account_owner: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub new_metadata_update_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for MintNewEditionFromMasterEditionViaToken {
    type ArrangedAccounts = MintNewEditionFromMasterEditionViaTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let new_metadata = next_account(&mut iter)?;
        let new_edition = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter)?;
        let new_mint = next_account(&mut iter)?;
        let edition_mark_pda = next_account(&mut iter)?;
        let new_mint_authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_account_owner = next_account(&mut iter)?;
        let token_account = next_account(&mut iter)?;
        let new_metadata_update_authority = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter);

        Some(MintNewEditionFromMasterEditionViaTokenInstructionAccounts {
            new_metadata,
            new_edition,
            master_edition,
            new_mint,
            edition_mark_pda,
            new_mint_authority,
            payer,
            token_account_owner,
            token_account,
            new_metadata_update_authority,
            metadata,
            token_program,
            system_program,
            rent,
        })
    }
}
