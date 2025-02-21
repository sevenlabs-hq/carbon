use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x7911d26b6deb0e0c")]
pub struct ZetaGroup {
    pub nonce: u8,
    pub nonce_padding: [u8; 2],
    pub front_expiry_index: u8,
    pub halt_state: HaltState,
    pub underlying_mint: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub greeks: solana_sdk::pubkey::Pubkey,
    pub pricing_parameters: PricingParameters,
    pub margin_parameters: MarginParameters,
    pub margin_parameters_padding: [u8; 104],
    pub products: [Product; 46],
    pub products_padding: [Product; 91],
    pub perp: Product,
    pub expiry_series: [ExpirySeries; 2],
    pub expiry_series_padding: [ExpirySeries; 4],
    pub deprecated_padding: [u8; 8],
    pub asset: Asset,
    pub expiry_interval_seconds: u32,
    pub new_expiry_threshold_seconds: u32,
    pub perp_parameters: PerpParameters,
    pub perp_sync_queue: solana_sdk::pubkey::Pubkey,
    pub oracle_backup_feed: solana_sdk::pubkey::Pubkey,
    pub perps_only: bool,
    pub flex_underlying: bool,
    pub padding: [u8; 964],
}
