
use super::*;
use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SetReservationListArgs {
    pub reservations: Vec<Reservation>,
    pub total_reservation_spots: Option<u64>,
    pub offset: u64,
    pub total_spot_offset: u64,
}

