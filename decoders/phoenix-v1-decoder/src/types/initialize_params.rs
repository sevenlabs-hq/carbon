use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeParams {
    pub market_size_params: MarketSizeParams,
    pub num_quote_lots_per_quote_unit: u64,
    pub tick_size_in_quote_lots_per_base_unit: u64,
    pub num_base_lots_per_base_unit: u64,
    pub taker_fee_bps: u16,
    pub fee_collector: solana_pubkey::Pubkey,
    pub raw_base_units_per_base_unit: Option<u32>,
}
