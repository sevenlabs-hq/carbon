use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum OrderBookType {
    Collection {
        collection_key: solana_pubkey::Pubkey,
    },
    NFTList {
        list_account: solana_pubkey::Pubkey,
    },
}
