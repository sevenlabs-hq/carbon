use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d83ee27301e1ba51c")]
pub struct WithdrawStakeAccountEvent {
    pub state: solana_sdk::pubkey::Pubkey,
    pub epoch: u64,
    pub stake: solana_sdk::pubkey::Pubkey,
    pub last_update_stake_delegation: u64,
    pub stake_index: u32,
    pub validator: solana_sdk::pubkey::Pubkey,
    pub validator_index: u32,
    pub user_msol_balance: u64,
    pub user_msol_auth: solana_sdk::pubkey::Pubkey,
    pub msol_burned: u64,
    pub msol_fees: u64,
    pub split_stake: solana_sdk::pubkey::Pubkey,
    pub beneficiary: solana_sdk::pubkey::Pubkey,
    pub split_lamports: u64,
    pub fee_bp_cents: u32,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
