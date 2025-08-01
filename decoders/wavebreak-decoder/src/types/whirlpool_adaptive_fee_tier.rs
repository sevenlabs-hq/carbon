use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WhirlpoolAdaptiveFeeTier {
    pub discriminator: [u8; 8],
    pub whirlpool_config: solana_pubkey::Pubkey,
    pub fee_tier_index: u16,
    pub tick_spacing: u16,
    pub initialize_pool_authority: solana_pubkey::Pubkey,
    pub delegated_fee_authority: solana_pubkey::Pubkey,
    pub default_base_fee_rate: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
}
