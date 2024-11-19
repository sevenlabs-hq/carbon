use super::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct VaultAllocation {
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub ctoken_vault: solana_sdk::pubkey::Pubkey,
    pub target_allocation_weight: u64,
    pub token_allocation_cap: u64,
    #[serde(with = "BigArray")]
    pub config_padding: [u64; 128],
    pub ctoken_allocation: u64,
    pub last_invest_slot: u64,
    pub token_target_allocation_sf: u128,
    #[serde(with = "BigArray")]
    pub state_padding: [u64; 128],
}
