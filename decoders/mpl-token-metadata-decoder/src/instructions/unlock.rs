use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2f")]
pub struct Unlock {
    pub unlock_args: UnlockArgs,
}

pub struct UnlockInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token_owner: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Unlock {
    type ArrangedAccounts = UnlockInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, token_owner, token, mint, metadata, edition, token_record, payer, system_program, sysvar_instructions, spl_token_program, authorization_rules_program, authorization_rules, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UnlockInstructionAccounts {
            authority: authority.pubkey,
            token_owner: token_owner.pubkey,
            token: token.pubkey,
            mint: mint.pubkey,
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            token_record: token_record.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}
