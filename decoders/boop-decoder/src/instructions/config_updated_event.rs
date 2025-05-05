use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df59e81633c64d6dc")]
pub struct ConfigUpdatedEvent {
    pub protocol_fee_recipient: solana_pubkey::Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub damping_term: u8,
    pub swap_fee_basis_points: u8,
    pub token_for_stakers_basis_points: u16,
    pub token_amount_for_raydium_liquidity: u64,
    pub max_graduation_price_deviation_basis_points: u16,
    pub max_swap_amount_for_pool_price_correction_basis_points: u16,
}
