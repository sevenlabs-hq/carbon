use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1c52153b968d3c7c")]
pub struct MarketNode {
    pub index: u8,
    pub nonce: u8,
    pub node_updates: [i64; 5],
    pub interest_update: i64,
}
