
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Observation {
    pub cumulative_active_bin_id: i128,
    pub created_at: i64,
    pub last_updated_at: i64,
}
