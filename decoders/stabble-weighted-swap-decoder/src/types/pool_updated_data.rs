use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolUpdatedData {
    pub is_active: bool,
    pub swap_fee: u64,
    pub max_supply: u64,
}
