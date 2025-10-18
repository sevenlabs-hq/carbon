use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BuyParams {
    pub max_sol_spend: u64,
    pub minimum_amount_out: u64,
    pub encoded_user_defined_event_data: String,
}
