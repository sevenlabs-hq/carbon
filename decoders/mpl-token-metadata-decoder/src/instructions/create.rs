use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2a")]
pub struct Create {
    pub create_args: CreateArgs,
}

pub struct CreateInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, master_edition, mint, authority, payer, update_authority, system_program, sysvar_instructions, spl_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateInstructionAccounts {
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
        })
    }
}
