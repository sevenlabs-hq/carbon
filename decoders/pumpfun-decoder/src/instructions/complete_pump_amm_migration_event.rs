use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbde95db95c94ea94")]
pub struct CompletePumpAmmMigrationEvent {
    pub user: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub mint_amount: u64,
    pub sol_amount: u64,
    pub pool_migration_fee: u64,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub timestamp: i64,
    pub pool: solana_pubkey::Pubkey,
}
