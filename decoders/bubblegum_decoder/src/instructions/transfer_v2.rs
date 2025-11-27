use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x772806ebeaddf831")]
pub struct TransferV2 {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub asset_data_hash: Option<[u8; 32]>,
    pub flags: Option<u8>,
    pub nonce: u64,
    pub index: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferV2InstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub leaf_delegate: solana_pubkey::Pubkey,
    pub new_leaf_owner: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub core_collection: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferV2 {
    type ArrangedAccounts = TransferV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, payer, authority, leaf_owner, leaf_delegate, new_leaf_owner, merkle_tree, core_collection, log_wrapper, compression_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferV2InstructionAccounts {
            tree_authority: tree_authority.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            new_leaf_owner: new_leaf_owner.pubkey,
            merkle_tree: merkle_tree.pubkey,
            core_collection: core_collection.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
