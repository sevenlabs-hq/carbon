use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x31")]
pub struct Transfer {
    pub transfer_args: TransferArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub token_owner: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub destination_owner: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub edition: Option<solana_pubkey::Pubkey>,
    pub owner_token_record: Option<solana_pubkey::Pubkey>,
    pub destination_token_record: Option<solana_pubkey::Pubkey>,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub spl_ata_program: solana_pubkey::Pubkey,
    pub authorization_rules_program: Option<solana_pubkey::Pubkey>,
    pub authorization_rules: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token = next_account(&mut iter)?;
        let token_owner = next_account(&mut iter)?;
        let destination = next_account(&mut iter)?;
        let destination_owner = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter);
        let owner_token_record = next_account(&mut iter);
        let destination_token_record = next_account(&mut iter);
        let authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter)?;
        let spl_ata_program = next_account(&mut iter)?;
        let authorization_rules_program = next_account(&mut iter);
        let authorization_rules = next_account(&mut iter);

        Some(TransferInstructionAccounts {
            token,
            token_owner,
            destination,
            destination_owner,
            mint,
            metadata,
            edition,
            owner_token_record,
            destination_token_record,
            authority,
            payer,
            system_program,
            sysvar_instructions,
            spl_token_program,
            spl_ata_program,
            authorization_rules_program,
            authorization_rules,
        })
    }
}
