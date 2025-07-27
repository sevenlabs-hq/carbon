use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2d")]
pub struct Revoke {
    pub revoke_args: RevokeArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RevokeInstructionAccounts {
    pub delegate_record: Option<solana_pubkey::Pubkey>,
    pub delegate: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub master_edition: Option<solana_pubkey::Pubkey>,
    pub token_record: Option<solana_pubkey::Pubkey>,
    pub mint: solana_pubkey::Pubkey,
    pub token: Option<solana_pubkey::Pubkey>,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub spl_token_program: Option<solana_pubkey::Pubkey>,
    pub authorization_rules_program: Option<solana_pubkey::Pubkey>,
    pub authorization_rules: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for Revoke {
    type ArrangedAccounts = RevokeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let delegate_record = next_account(&mut iter);
        let delegate = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter);
        let token_record = next_account(&mut iter);
        let mint = next_account(&mut iter)?;
        let token = next_account(&mut iter);
        let authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter);
        let authorization_rules_program = next_account(&mut iter);
        let authorization_rules = next_account(&mut iter);

        Some(RevokeInstructionAccounts {
            delegate_record,
            delegate,
            metadata,
            master_edition,
            token_record,
            mint,
            token,
            authority,
            payer,
            system_program,
            sysvar_instructions,
            spl_token_program,
            authorization_rules_program,
            authorization_rules,
        })
    }
}
