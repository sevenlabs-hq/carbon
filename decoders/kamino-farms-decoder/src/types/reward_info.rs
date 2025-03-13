use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RewardInfo {
    pub token: TokenInfo,
    pub rewards_vault: solana_sdk::pubkey::Pubkey,
    pub rewards_available: u64,
    pub reward_schedule_curve: RewardScheduleCurve,
    pub min_claim_duration_seconds: u64,
    pub last_issuance_ts: u64,
    pub rewards_issued_unclaimed: u64,
    pub rewards_issued_cumulative: u64,
    pub reward_per_share_scaled: u128,
    pub placeholder0: u64,
    pub reward_type: u8,
    pub rewards_per_second_decimals: u8,
    pub padding0: [u8; 6],
    pub padding1: [u64; 20],
}
