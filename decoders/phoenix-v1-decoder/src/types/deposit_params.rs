use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DepositParams {
    pub quote_lots_to_deposit: u64,
    pub base_lots_to_deposit: u64,
}
