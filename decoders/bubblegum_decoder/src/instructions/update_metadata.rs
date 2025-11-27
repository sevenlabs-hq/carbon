use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaab62bef614ee1ba")]
pub struct UpdateMetadata {
    pub root: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub current_metadata: MetadataArgs,
    pub update_args: UpdateArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateMetadataInstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection_metadata: solana_pubkey::Pubkey,
    pub collection_authority_record_pda: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub leaf_delegate: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub token_metadata_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMetadata {
    type ArrangedAccounts = UpdateMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, authority, collection_mint, collection_metadata, collection_authority_record_pda, leaf_owner, leaf_delegate, payer, merkle_tree, log_wrapper, compression_program, token_metadata_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdateMetadataInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            authority: authority.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_authority_record_pda: collection_authority_record_pda.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            payer: payer.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
