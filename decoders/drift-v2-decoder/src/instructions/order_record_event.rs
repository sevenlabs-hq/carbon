use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d681340385915025a")]
pub struct OrderRecordEvent {
    pub ts: i64,
    pub user: solana_pubkey::Pubkey,
    pub order: Order,
}
