use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x29")]
pub struct Burn {
    pub burn_args: BurnArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BurnInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub collection_metadata: Option<solana_pubkey::Pubkey>,
    pub metadata: solana_pubkey::Pubkey,
    pub edition: Option<solana_pubkey::Pubkey>,
    pub mint: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub master_edition: Option<solana_pubkey::Pubkey>,
    pub master_edition_mint: Option<solana_pubkey::Pubkey>,
    pub master_edition_token: Option<solana_pubkey::Pubkey>,
    pub edition_marker: Option<solana_pubkey::Pubkey>,
    pub token_record: Option<solana_pubkey::Pubkey>,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Burn {
    type ArrangedAccounts = BurnInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let collection_metadata = next_account(&mut iter);
        let metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter);
        let mint = next_account(&mut iter)?;
        let token = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter);
        let master_edition_mint = next_account(&mut iter);
        let master_edition_token = next_account(&mut iter);
        let edition_marker = next_account(&mut iter);
        let token_record = next_account(&mut iter);
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter)?;

        Some(BurnInstructionAccounts {
            authority,
            collection_metadata,
            metadata,
            edition,
            mint,
            token,
            master_edition,
            master_edition_mint,
            master_edition_token,
            edition_marker,
            token_record,
            system_program,
            sysvar_instructions,
            spl_token_program,
        })
    }
}
