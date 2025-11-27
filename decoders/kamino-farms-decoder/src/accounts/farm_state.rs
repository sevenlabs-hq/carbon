use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc666d84a3f42a3be")]
pub struct FarmState {
    pub farm_admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub token: TokenInfo,
    pub reward_infos: [RewardInfo; 10],
    pub num_reward_tokens: u64,
    pub num_users: u64,
    pub total_staked_amount: u64,
    pub farm_vault: solana_pubkey::Pubkey,
    pub farm_vaults_authority: solana_pubkey::Pubkey,
    pub farm_vaults_authority_bump: u64,
    pub delegate_authority: solana_pubkey::Pubkey,
    pub time_unit: u8,
    pub is_farm_frozen: u8,
    pub is_farm_delegated: u8,
    pub padding0: [u8; 5],
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub deposit_warmup_period: u32,
    pub withdrawal_cooldown_period: u32,
    pub total_active_stake_scaled: u128,
    pub total_pending_stake_scaled: u128,
    pub total_pending_amount: u64,
    pub slashed_amount_current: u64,
    pub slashed_amount_cumulative: u64,
    pub slashed_amount_spill_address: solana_pubkey::Pubkey,
    pub locking_mode: u64,
    pub locking_start_timestamp: u64,
    pub locking_duration: u64,
    pub locking_early_withdrawal_penalty_bps: u64,
    pub deposit_cap_amount: u64,
    pub scope_prices: solana_pubkey::Pubkey,
    pub scope_oracle_price_id: u64,
    pub scope_oracle_max_age: u64,
    pub pending_farm_admin: solana_pubkey::Pubkey,
    pub strategy_id: solana_pubkey::Pubkey,
    pub delegated_rps_admin: solana_pubkey::Pubkey,
    pub vault_id: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u64; 78],
}
