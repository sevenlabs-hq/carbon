use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dfb12804bd050ae8c")]
pub struct UpdateActiveEvent {
    pub state: solana_sdk::pubkey::Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub validator_index: u32,
    pub validator_vote: solana_sdk::pubkey::Pubkey,
    pub delegation_change: U64ValueChange,
    pub delegation_growth_msol_fees: Option<u64>,
    pub extra_lamports: u64,
    pub extra_msol_fees: Option<u64>,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub msol_price_change: U64ValueChange,
    pub reward_fee_used: Fee,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
