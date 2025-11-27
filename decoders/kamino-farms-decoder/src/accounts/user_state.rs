use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x48b155f94ca7ba7e")]
pub struct UserState {
    pub user_id: u64,
    pub farm_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub is_farm_delegated: u8,
    pub padding0: [u8; 7],
    pub rewards_tally_scaled: [u128; 10],
    pub rewards_issued_unclaimed: [u64; 10],
    pub last_claim_ts: [u64; 10],
    pub active_stake_scaled: u128,
    pub pending_deposit_stake_scaled: u128,
    pub pending_deposit_stake_ts: u64,
    pub pending_withdrawal_unstake_scaled: u128,
    pub pending_withdrawal_unstake_ts: u64,
    pub bump: u64,
    pub delegatee: solana_pubkey::Pubkey,
    pub last_stake_ts: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding1: [u64; 50],
}
