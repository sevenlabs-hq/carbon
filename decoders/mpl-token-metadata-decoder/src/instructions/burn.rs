use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x29")]
pub struct Burn {
    pub burn_args: BurnArgs,
}

pub struct BurnInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub master_edition_mint: solana_sdk::pubkey::Pubkey,
    pub master_edition_token: solana_sdk::pubkey::Pubkey,
    pub edition_marker: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Burn {
    type ArrangedAccounts = BurnInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, collection_metadata, metadata, edition, mint, token, master_edition, master_edition_mint, master_edition_token, edition_marker, token_record, system_program, sysvar_instructions, spl_token_program] =
            accounts
        else {
            return None;
        };

        Some(BurnInstructionAccounts {
            authority: authority.pubkey,
            collection_metadata: collection_metadata.pubkey,
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            mint: mint.pubkey,
            token: token.pubkey,
            master_edition: master_edition.pubkey,
            master_edition_mint: master_edition_mint.pubkey,
            master_edition_token: master_edition_token.pubkey,
            edition_marker: edition_marker.pubkey,
            token_record: token_record.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
        })
    }
}
