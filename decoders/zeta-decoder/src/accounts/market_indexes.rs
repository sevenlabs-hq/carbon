use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6fcd6992db895217")]
pub struct MarketIndexes {
    pub nonce: u8,
    pub initialized: bool,
    #[serde(with = "serde_big_array::BigArray")]
    pub indexes: [u8; 138],
}
