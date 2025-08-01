use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MetaplexCreator {
    pub address: solana_pubkey::Pubkey,
    pub verified: bool,
    pub share: u8,
}
