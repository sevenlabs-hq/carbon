use carbon_core::{borsh, CarbonDeserialize};
#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xa7e8e8b1c86c727f")]
pub struct Global {
    pub initialized: bool,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub fee_recipient: solana_sdk::pubkey::Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
}
