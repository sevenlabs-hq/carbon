
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Oracle {
    pub base_address: solana_sdk::pubkey::Pubkey,
    pub base_address_config: Option<ExtraAccount>,
    pub results_offset: ValidationResultsOffset,
}
