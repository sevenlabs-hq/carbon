use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa7e8e8b1c86c727f")]
#[derive(Default)]
pub struct Global {
    pub initialized: bool,
    pub authority: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub fee_recipients: [solana_pubkey::Pubkey; 7],
    pub set_creator_authority: solana_pubkey::Pubkey,
}

/*
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]


#[carbon(discriminator = "0xa7e8e8b1c86c727f")]
pub struct Global {
        pub initialized: bool,
        pub authority: solana_pubkey::Pubkey,
        pub fee_recipient: solana_pubkey::Pubkey,
        pub initial_virtual_token_reserves: u64,
        pub initial_virtual_sol_reserves: u64,
        pub initial_real_token_reserves: u64,
        pub token_total_supply: u64,
        pub fee_basis_points: u64,
        pub withdraw_authority: solana_pubkey::Pubkey,
        pub enable_migrate: bool,
        pub pool_migration_fee: u64,
        pub creator_fee_basis_points: u64,
        pub fee_recipients: [solana_pubkey::Pubkey; 7],
        pub set_creator_authority: solana_pubkey::Pubkey,
}
*/
