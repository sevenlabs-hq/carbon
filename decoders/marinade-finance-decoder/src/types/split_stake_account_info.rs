

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SplitStakeAccountInfo {
    pub account: solana_sdk::pubkey::Pubkey,
    pub index: u32,
}
