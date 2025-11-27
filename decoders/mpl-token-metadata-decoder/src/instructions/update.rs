use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x32")]
pub struct Update {
    pub update_args: UpdateArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub delegate_record: Option<solana_pubkey::Pubkey>,
    pub token: Option<solana_pubkey::Pubkey>,
    pub mint: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub edition: Option<solana_pubkey::Pubkey>,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub authorization_rules_program: Option<solana_pubkey::Pubkey>,
    pub authorization_rules: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for Update {
    type ArrangedAccounts = UpdateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let delegate_record = next_account(&mut iter);
        let token = next_account(&mut iter);
        let mint = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter);
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let authorization_rules_program = next_account(&mut iter);
        let authorization_rules = next_account(&mut iter);

        Some(UpdateInstructionAccounts {
            authority,
            delegate_record,
            token,
            mint,
            metadata,
            edition,
            payer,
            system_program,
            sysvar_instructions,
            authorization_rules_program,
            authorization_rules,
        })
    }
}
