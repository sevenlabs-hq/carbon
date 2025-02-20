use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x5c0e8bea48f4441a")]
pub struct PrelaunchOracle {
    pub price: i64,
    pub max_price: i64,
    pub confidence: u64,
    pub last_update_slot: u64,
    pub amm_last_update_slot: u64,
    pub perp_market_index: u16,
    pub padding: [u8; 70],
}
