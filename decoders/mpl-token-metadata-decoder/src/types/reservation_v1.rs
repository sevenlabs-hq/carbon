

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ReservationV1 {
    pub address: solana_sdk::pubkey::Pubkey,
    pub spots_remaining: u8,
    pub total_spots: u8,
}
