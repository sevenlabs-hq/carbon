use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct Global {
    pub _padding: [u8; 8],
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
    pub creator_fee: u64,
    pub fee_recipients: [solana_pubkey::Pubkey; 7],
    pub _reserved: [u8; 399],
}

impl Default for Global {
    fn default() -> Self {
        Self {
            _padding: [0; 8],
            initialized: false,
            authority: solana_pubkey::Pubkey::default(),
            fee_recipient: solana_pubkey::Pubkey::default(),
            initial_virtual_token_reserves: 0,
            initial_virtual_sol_reserves: 0,
            initial_real_token_reserves: 0,
            token_total_supply: 0,
            fee_basis_points: 0,
            withdraw_authority: solana_pubkey::Pubkey::default(),
            enable_migrate: false,
            pool_migration_fee: 0,
            creator_fee: 0,
            fee_recipients: [
                solana_pubkey::Pubkey::default(),
                solana_pubkey::Pubkey::default(),
                solana_pubkey::Pubkey::default(),
                solana_pubkey::Pubkey::default(),
                solana_pubkey::Pubkey::default(),
                solana_pubkey::Pubkey::default(),
                solana_pubkey::Pubkey::default(),
            ],
            _reserved: [0; 399],
        }
    }
}
