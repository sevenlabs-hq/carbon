use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9912b22fc59e560f")]
pub struct MintToCollectionV1 {
    pub metadata_args: MetadataArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MintToCollectionV1InstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub leaf_delegate: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub tree_delegate: solana_pubkey::Pubkey,
    pub collection_authority: solana_pubkey::Pubkey,
    pub collection_authority_record_pda: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection_metadata: solana_pubkey::Pubkey,
    pub edition_account: solana_pubkey::Pubkey,
    pub bubblegum_signer: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub token_metadata_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintToCollectionV1 {
    type ArrangedAccounts = MintToCollectionV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, leaf_owner, leaf_delegate, merkle_tree, payer, tree_delegate, collection_authority, collection_authority_record_pda, collection_mint, collection_metadata, edition_account, bubblegum_signer, log_wrapper, compression_program, token_metadata_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MintToCollectionV1InstructionAccounts {
            tree_authority: tree_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            payer: payer.pubkey,
            tree_delegate: tree_delegate.pubkey,
            collection_authority: collection_authority.pubkey,
            collection_authority_record_pda: collection_authority_record_pda.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            edition_account: edition_account.pubkey,
            bubblegum_signer: bubblegum_signer.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
