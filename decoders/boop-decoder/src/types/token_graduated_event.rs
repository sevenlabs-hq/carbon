use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenGraduatedEvent {
    pub mint: solana_pubkey::Pubkey,
    pub sol_for_liquidity: u64,
    pub graduation_fee: u64,
    pub token_for_distributor: u64,
}
