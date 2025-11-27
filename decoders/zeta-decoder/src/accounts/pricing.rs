use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbe7bd2b68f0b9888")]
pub struct Pricing {
    pub nonce: u8,
    pub mark_prices: [u64; 25],
    pub mark_prices_padding: [u64; 0],
    pub update_timestamps: [u64; 25],
    pub update_timestamps_padding: [u64; 0],
    pub funding_deltas: [AnchorDecimal; 25],
    pub funding_deltas_padding: [AnchorDecimal; 0],
    pub latest_funding_rates: [AnchorDecimal; 25],
    pub latest_funding_rates_padding: [AnchorDecimal; 0],
    pub latest_midpoints: [u64; 25],
    pub latest_midpoints_padding: [u64; 0],
    pub oracles: [solana_pubkey::Pubkey; 25],
    pub oracles_padding: [solana_pubkey::Pubkey; 0],
    pub oracle_backup_feeds: [solana_pubkey::Pubkey; 25],
    pub oracle_backup_feeds_padding: [solana_pubkey::Pubkey; 0],
    pub markets: [solana_pubkey::Pubkey; 25],
    pub markets_padding: [solana_pubkey::Pubkey; 0],
    pub perp_sync_queues: [solana_pubkey::Pubkey; 25],
    pub perp_sync_queues_padding: [solana_pubkey::Pubkey; 0],
    pub perp_parameters: [PerpParameters; 25],
    pub perp_parameters_padding: [PerpParameters; 0],
    pub margin_parameters: [MarginParameters; 25],
    pub margin_parameters_padding: [MarginParameters; 0],
    pub products: [Product; 25],
    pub products_padding: [Product; 0],
    pub zeta_group_keys: [solana_pubkey::Pubkey; 25],
    pub zeta_group_keys_padding: [solana_pubkey::Pubkey; 0],
    pub total_insurance_vault_deposits: u64,
    pub last_withdraw_timestamp: u64,
    pub net_outflow_sum: i64,
    pub halt_force_pricing: [bool; 25],
    pub halt_force_pricing_padding: [bool; 0],
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 2707],
}
