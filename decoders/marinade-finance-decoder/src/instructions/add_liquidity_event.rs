

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1bb299ba2fc48c2d")]
pub struct AddLiquidityEvent{
    pub state: solana_sdk::pubkey::Pubkey,
    pub sol_owner: solana_sdk::pubkey::Pubkey,
    pub user_sol_balance: u64,
    pub user_lp_balance: u64,
    pub sol_leg_balance: u64,
    pub lp_supply: u64,
    pub sol_added_amount: u64,
    pub lp_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
