use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dad05930f050ec274")]
pub struct LiquidUnstakeEvent {
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_owner: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_balance: u64,
    pub liq_pool_msol_balance: u64,
    pub treasury_msol_balance: Option<u64>,
    pub user_msol_balance: u64,
    pub user_sol_balance: u64,
    pub msol_amount: u64,
    pub msol_fee: u64,
    pub treasury_msol_cut: u64,
    pub sol_amount: u64,
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub treasury_cut: Fee,
}
