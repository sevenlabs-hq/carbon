use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d0236b8da4eb5a375")]
pub struct DeactivateStakeEvent {
    pub state: solana_sdk::pubkey::Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub last_update_stake_delegation: u64,
    pub split_stake_account: Option<SplitStakeAccountInfo>,
    pub validator_index: u32,
    pub validator_vote: solana_sdk::pubkey::Pubkey,
    pub total_stake_target: u64,
    pub validator_stake_target: u64,
    pub total_active_balance: u64,
    pub delayed_unstake_cooling_down: u64,
    pub validator_active_balance: u64,
    pub total_unstake_delta: u64,
    pub unstaked_amount: u64,
}
