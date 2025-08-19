use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct HaltState {
    pub halted: bool,
    pub spot_price: u64,
    pub timestamp: u64,
    pub mark_prices_set: [bool; 2],
    pub mark_prices_set_padding: [bool; 3],
    pub perp_mark_price_set: bool,
    pub market_nodes_cleaned: [bool; 2],
    pub market_nodes_cleaned_padding: [bool; 4],
    #[serde(with = "BigArray")]
    pub market_cleaned: [bool; 46],
    #[serde(with = "BigArray")]
    pub market_cleaned_padding: [bool; 91],
    pub perp_market_cleaned: bool,
}
