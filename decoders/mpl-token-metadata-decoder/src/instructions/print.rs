use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x37")]
pub struct Print {
    pub print_args: PrintArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PrintInstructionAccounts {
    pub edition_metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub edition_mint: solana_pubkey::Pubkey,
    pub edition_token_account_owner: solana_pubkey::Pubkey,
    pub edition_token_account: solana_pubkey::Pubkey,
    pub edition_mint_authority: solana_pubkey::Pubkey,
    pub edition_token_record: Option<solana_pubkey::Pubkey>,
    pub master_edition: solana_pubkey::Pubkey,
    pub edition_marker_pda: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub master_token_account_owner: solana_pubkey::Pubkey,
    pub master_token_account: solana_pubkey::Pubkey,
    pub master_metadata: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub spl_ata_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Print {
    type ArrangedAccounts = PrintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let edition_metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter)?;
        let edition_mint = next_account(&mut iter)?;
        let edition_token_account_owner = next_account(&mut iter)?;
        let edition_token_account = next_account(&mut iter)?;
        let edition_mint_authority = next_account(&mut iter)?;
        let edition_token_record = next_account(&mut iter);
        let master_edition = next_account(&mut iter)?;
        let edition_marker_pda = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let master_token_account_owner = next_account(&mut iter)?;
        let master_token_account = next_account(&mut iter)?;
        let master_metadata = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter)?;
        let spl_ata_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(PrintInstructionAccounts {
            edition_metadata,
            edition,
            edition_mint,
            edition_token_account_owner,
            edition_token_account,
            edition_mint_authority,
            edition_token_record,
            master_edition,
            edition_marker_pda,
            payer,
            master_token_account_owner,
            master_token_account,
            master_metadata,
            update_authority,
            spl_token_program,
            spl_ata_program,
            sysvar_instructions,
            system_program,
        })
    }
}
