use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddLiquidityIxParams {
    pub desired_base_amount_in: u64,
    pub desired_quote_amount_in: u64,
    pub initial_lp_shares: Option<u64>,
}
