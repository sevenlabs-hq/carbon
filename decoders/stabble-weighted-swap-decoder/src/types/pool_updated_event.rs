use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolUpdatedEvent {
    pub pubkey: solana_pubkey::Pubkey,
    pub data: PoolUpdatedData,
}
