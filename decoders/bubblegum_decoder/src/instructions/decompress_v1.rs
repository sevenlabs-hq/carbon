use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x36554c46e4faa451")]
pub struct DecompressV1 {
    pub metadata: MetadataArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecompressV1InstructionAccounts {
    pub voucher: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_rent: solana_pubkey::Pubkey,
    pub token_metadata_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecompressV1 {
    type ArrangedAccounts = DecompressV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [voucher, leaf_owner, token_account, mint, mint_authority, metadata, master_edition, system_program, sysvar_rent, token_metadata_program, token_program, associated_token_program, log_wrapper, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DecompressV1InstructionAccounts {
            voucher: voucher.pubkey,
            leaf_owner: leaf_owner.pubkey,
            token_account: token_account.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            system_program: system_program.pubkey,
            sysvar_rent: sysvar_rent.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
