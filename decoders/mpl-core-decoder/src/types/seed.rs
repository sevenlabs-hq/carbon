use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum Seed {
    Collection,
    Owner,
    Recipient,
    Asset,
    Address(solana_pubkey::Pubkey),
    Bytes(Vec<u8>),
}
