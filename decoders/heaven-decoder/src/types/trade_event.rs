use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TradeEvent {
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub total_creator_trading_fees: u64,
    pub total_fee_paid: u64,
}
