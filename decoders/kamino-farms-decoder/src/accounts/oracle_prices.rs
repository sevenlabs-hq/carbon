use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x598076dd0648b492")]
pub struct OraclePrices {
    pub oracle_mappings: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub prices: [DatedPrice; 512],
}
