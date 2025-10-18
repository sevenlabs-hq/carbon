use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolSlotOffsetBasedFees {
    pub protocol_trading_fee: SlotFeeBrackets,
    pub liquidity_provider_trading_fee: SlotFeeBrackets,
    pub creator_trading_fee: SlotFeeBrackets,
    pub creator_trading_fee_protocol_fee: SlotFeeBrackets,
    pub reflection_trading_fee: SlotFeeBrackets,
    pub pad: [u8; 6],
}
