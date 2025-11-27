use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x56d6be25a7041c74")]
pub struct ThawAndRevokeV2 {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub collection_hash: Option<[u8; 32]>,
    pub asset_data_hash: Option<[u8; 32]>,
    pub flags: Option<u8>,
    pub nonce: u64,
    pub index: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ThawAndRevokeV2InstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub leaf_delegate: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ThawAndRevokeV2 {
    type ArrangedAccounts = ThawAndRevokeV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, payer, leaf_delegate, leaf_owner, merkle_tree, log_wrapper, compression_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ThawAndRevokeV2InstructionAccounts {
            tree_authority: tree_authority.pubkey,
            payer: payer.pubkey,
            leaf_delegate: leaf_delegate.pubkey,
            leaf_owner: leaf_owner.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
