use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ReserveCollateral {
    pub mint_pubkey: solana_pubkey::Pubkey,
    pub mint_total_supply: u64,
    pub supply_vault: solana_pubkey::Pubkey,
    pub padding1: [u128; 32],
    pub padding2: [u128; 32],
}
