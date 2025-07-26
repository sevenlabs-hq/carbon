use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xef4f0cce7499018c")]
pub struct ReservationListV1 {
    pub key: Key,
    pub master_edition: solana_pubkey::Pubkey,
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<ReservationV1>,
}
