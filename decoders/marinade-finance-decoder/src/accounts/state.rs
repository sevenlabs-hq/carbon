use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xd8926b5e684bb6b1")]
pub struct State {
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub operational_sol_account: solana_sdk::pubkey::Pubkey,
    pub treasury_msol_account: solana_sdk::pubkey::Pubkey,
    pub reserve_bump_seed: u8,
    pub msol_mint_authority_bump_seed: u8,
    pub rent_exempt_for_token_acc: u64,
    pub reward_fee: Fee,
    pub stake_system: StakeSystem,
    pub validator_system: ValidatorSystem,
    pub liq_pool: LiqPool,
    pub available_reserve_balance: u64,
    pub msol_supply: u64,
    pub msol_price: u64,
    pub circulating_ticket_count: u64,
    pub circulating_ticket_balance: u64,
    pub lent_from_reserve: u64,
    pub min_deposit: u64,
    pub min_withdraw: u64,
    pub staking_sol_cap: u64,
    pub emergency_cooling_down: u64,
    pub pause_authority: solana_sdk::pubkey::Pubkey,
    pub paused: bool,
    pub delayed_unstake_fee: FeeCents,
    pub withdraw_stake_account_fee: FeeCents,
    pub withdraw_stake_account_enabled: bool,
    pub last_stake_move_epoch: u64,
    pub stake_moved: u64,
    pub max_stake_moved_per_epoch: Fee,
}
