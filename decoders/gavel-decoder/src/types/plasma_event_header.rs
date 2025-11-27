use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlasmaEventHeader {
    pub sequence_number: u64,
    pub slot: u64,
    pub timestamp: i64,
    pub pool: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}
