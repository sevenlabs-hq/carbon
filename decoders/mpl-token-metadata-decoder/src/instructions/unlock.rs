use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2f")]
pub struct Unlock {
    pub unlock_args: UnlockArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UnlockInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub token_owner: Option<solana_pubkey::Pubkey>,
    pub token: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub edition: Option<solana_pubkey::Pubkey>,
    pub token_record: Option<solana_pubkey::Pubkey>,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub spl_token_program: Option<solana_pubkey::Pubkey>,
    pub authorization_rules_program: Option<solana_pubkey::Pubkey>,
    pub authorization_rules: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for Unlock {
    type ArrangedAccounts = UnlockInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let token_owner = next_account(&mut iter);
        let token = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter);
        let token_record = next_account(&mut iter);
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter);
        let authorization_rules_program = next_account(&mut iter);
        let authorization_rules = next_account(&mut iter);

        Some(UnlockInstructionAccounts {
            authority,
            token_owner,
            token,
            mint,
            metadata,
            edition,
            token_record,
            payer,
            system_program,
            sysvar_instructions,
            spl_token_program,
            authorization_rules_program,
            authorization_rules,
        })
    }
}
