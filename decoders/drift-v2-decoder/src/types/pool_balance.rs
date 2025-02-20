use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolBalance {
    pub scaled_balance: u128,
    pub market_index: u16,
    pub padding: [u8; 6],
}
