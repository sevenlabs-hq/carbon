use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa334c8e78c0345ba")]
pub struct Transfer {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferInstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub leaf_delegate: solana_pubkey::Pubkey,
    pub new_leaf_owner: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, leaf_owner, leaf_delegate, new_leaf_owner, merkle_tree, log_wrapper, compression_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            new_leaf_owner: new_leaf_owner.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
