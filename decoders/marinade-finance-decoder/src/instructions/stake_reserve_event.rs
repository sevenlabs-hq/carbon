use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d707595b94d77be6a")]
pub struct StakeReserveEvent {
    pub state: solana_pubkey::Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: solana_pubkey::Pubkey,
    pub validator_index: u32,
    pub validator_vote: solana_pubkey::Pubkey,
    pub total_stake_target: u64,
    pub validator_stake_target: u64,
    pub reserve_balance: u64,
    pub total_active_balance: u64,
    pub validator_active_balance: u64,
    pub total_stake_delta: u64,
    pub amount: u64,
}
