use carbon_core::{borsh, CarbonDeserialize};

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
