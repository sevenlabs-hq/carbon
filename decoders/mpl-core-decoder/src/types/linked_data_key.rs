use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum LinkedDataKey {
    LinkedLifecycleHook(solana_sdk::pubkey::Pubkey),
    LinkedAppData(Authority),
}
