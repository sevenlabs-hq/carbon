use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TradeParams {
    pub token_amount: u64,
    pub collateral_amount: u64,
    pub fixed_side: u8,
    pub slippage_bps: u64,
}
