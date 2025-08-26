use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SellParams {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub encoded_user_defined_event_data: String,
}
