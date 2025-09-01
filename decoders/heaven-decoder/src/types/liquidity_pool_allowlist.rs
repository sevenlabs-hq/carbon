use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolAllowlist {
    pub swap: u8,
    pub remove_liquidity: u8,
    pub deposit_liquidity: u8,
    pub same_slot_trading: u8,
    pub update_creator_trading_fee: u8,
    pub padding1: [u8; 2],
}
