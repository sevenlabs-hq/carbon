use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d681340385915025a")]
pub struct OrderRecordEvent {
    pub ts: i64,
    pub user: solana_pubkey::Pubkey,
    pub order: Order,
}
