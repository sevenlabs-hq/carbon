use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ObservationState {
    pub initialized: bool,
    pub observation_index: u16,
    pub pool_id: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub observations: [Observation; 100],
    pub padding: [u64; 4],
}
