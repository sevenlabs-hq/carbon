use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateStandardLiquidityPoolParams {
    pub encoded_user_defined_event_data: String,
    pub initial_purchase_amount: u64,
    pub max_sol_spend: u64,
}
