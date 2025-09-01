use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolMarketCapBasedFees {
    pub protocol_trading_fee: FeeBrackets,
    pub liquidity_provider_trading_fee: FeeBrackets,
    pub creator_trading_fee: FeeBrackets,
    pub creator_trading_fee_protocol_fee: FeeBrackets,
    pub reflection_trading_fee: FeeBrackets,
}
