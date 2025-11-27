use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe5233d5b0f0e63a0")]
pub struct SetCollectionV2 {
    pub root: [u8; 32],
    pub asset_data_hash: Option<[u8; 32]>,
    pub flags: Option<u8>,
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgsV2,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetCollectionV2InstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub new_collection_authority: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub leaf_delegate: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub core_collection: solana_pubkey::Pubkey,
    pub new_core_collection: solana_pubkey::Pubkey,
    pub mpl_core_cpi_signer: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollectionV2 {
    type ArrangedAccounts = SetCollectionV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, payer, authority, new_collection_authority, leaf_owner, leaf_delegate, merkle_tree, core_collection, new_core_collection, mpl_core_cpi_signer, log_wrapper, compression_program, mpl_core_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetCollectionV2InstructionAccounts {
            tree_authority: tree_authority.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            new_collection_authority: new_collection_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            core_collection: core_collection.pubkey,
            new_core_collection: new_core_collection.pubkey,
            mpl_core_cpi_signer: mpl_core_cpi_signer.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
