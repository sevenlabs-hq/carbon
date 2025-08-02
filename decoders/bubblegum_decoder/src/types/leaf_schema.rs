use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum LeafSchema {
    V1 {
        id: solana_pubkey::Pubkey,
        owner: solana_pubkey::Pubkey,
        delegate: solana_pubkey::Pubkey,
        nonce: u64,
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
    },
    V2 {
        id: solana_pubkey::Pubkey,
        owner: solana_pubkey::Pubkey,
        delegate: solana_pubkey::Pubkey,
        nonce: u64,
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        collection_hash: [u8; 32],
        asset_data_hash: [u8; 32],
        flags: u8,
    },
}
