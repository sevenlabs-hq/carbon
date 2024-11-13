 
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xc1e96137f58767da")] 
pub struct ReservationListV2 { 
        pub key: Key, 
        pub master_edition: solana_sdk::pubkey::Pubkey, 
        pub supply_snapshot: Option<u64>, 
        pub reservations: Vec<Reservation>, 
        pub total_reservation_spots: u64, 
        pub current_reservation_spots: u64, 
}
