use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MarketHeader {
    pub discriminant: u64,
    pub status: u64,
    pub market_size_params: MarketSizeParams,
    pub base_params: TokenParams,
    pub base_lot_size: u64,
    pub quote_params: TokenParams,
    pub quote_lot_size: u64,
    pub tick_size_in_quote_atoms_per_base_unit: u64,
    pub authority: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub market_sequence_number: u64,
    pub successor: solana_pubkey::Pubkey,
    pub raw_base_units_per_base_unit: u32,
    pub padding1: u32,
    pub padding2: [u64; 32],
}
