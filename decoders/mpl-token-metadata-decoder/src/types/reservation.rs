

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Reservation {
    pub address: solana_sdk::pubkey::Pubkey,
    pub spots_remaining: u64,
    pub total_spots: u64,
}
