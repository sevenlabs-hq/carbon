
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct OracleConfig {
    pub setup: OracleSetup,
    pub keys: [solana_sdk::pubkey::Pubkey; 5],
}
