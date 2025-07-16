use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x512a686f7b5992b4")]
pub struct SettlementAccount {
    pub settlement_price: u64,
    pub strikes: [u64; 23],
}
