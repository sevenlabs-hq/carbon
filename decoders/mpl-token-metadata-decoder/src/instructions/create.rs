use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2a")]
pub struct Create {
    pub create_args: CreateArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub master_edition: Option<solana_pubkey::Pubkey>,
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub spl_token_program: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter);
        let mint = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter);

        Some(CreateInstructionAccounts {
            metadata,
            master_edition,
            mint,
            authority,
            payer,
            update_authority,
            system_program,
            sysvar_instructions,
            spl_token_program,
        })
    }
}
