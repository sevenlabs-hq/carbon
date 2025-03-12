
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ConfigLpParams {
    pub min_fee: Option<Fee>,
    pub max_fee: Option<Fee>,
    pub liquidity_target: Option<u64>,
    pub treasury_cut: Option<Fee>,
}
