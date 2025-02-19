use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
pub struct ReservationListV2 {
    pub key: Key,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<Reservation>,
    pub total_reservation_spots: u64,
    pub current_reservation_spots: u64,
}
