use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x37")]
pub struct Print {
    pub print_args: PrintArgs,
}

pub struct PrintInstructionAccounts {
    pub edition_metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub edition_mint: solana_sdk::pubkey::Pubkey,
    pub edition_token_account_owner: solana_sdk::pubkey::Pubkey,
    pub edition_token_account: solana_sdk::pubkey::Pubkey,
    pub edition_mint_authority: solana_sdk::pubkey::Pubkey,
    pub edition_token_record: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub edition_marker_pda: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub master_token_account_owner: solana_sdk::pubkey::Pubkey,
    pub master_token_account: solana_sdk::pubkey::Pubkey,
    pub master_metadata: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub spl_ata_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Print {
    type ArrangedAccounts = PrintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [edition_metadata, edition, edition_mint, edition_token_account_owner, edition_token_account, edition_mint_authority, edition_token_record, master_edition, edition_marker_pda, payer, master_token_account_owner, master_token_account, master_metadata, update_authority, spl_token_program, spl_ata_program, sysvar_instructions, system_program] =
            accounts
        else {
            return None;
        };

        Some(PrintInstructionAccounts {
            edition_metadata: edition_metadata.pubkey,
            edition: edition.pubkey,
            edition_mint: edition_mint.pubkey,
            edition_token_account_owner: edition_token_account_owner.pubkey,
            edition_token_account: edition_token_account.pubkey,
            edition_mint_authority: edition_mint_authority.pubkey,
            edition_token_record: edition_token_record.pubkey,
            master_edition: master_edition.pubkey,
            edition_marker_pda: edition_marker_pda.pubkey,
            payer: payer.pubkey,
            master_token_account_owner: master_token_account_owner.pubkey,
            master_token_account: master_token_account.pubkey,
            master_metadata: master_metadata.pubkey,
            update_authority: update_authority.pubkey,
            spl_token_program: spl_token_program.pubkey,
            spl_ata_program: spl_ata_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
