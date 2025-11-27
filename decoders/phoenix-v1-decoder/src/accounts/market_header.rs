use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3869bff2e2f3c6a4")]
pub struct MarketHeader {
    pub discriminant: u64,
    pub status: u64,
    pub market_size_params: MarketSizeParams,
    pub base_params: TokenParams,
    pub base_lot_size: BaseAtomsPerBaseLot,
    pub quote_params: TokenParams,
    pub quote_lot_size: QuoteAtomsPerQuoteLot,
    pub tick_size_in_quote_atoms_per_base_unit: QuoteAtomsPerBaseUnitPerTick,
    pub authority: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub market_sequence_number: u64,
    pub successor: solana_pubkey::Pubkey,
    pub raw_base_units_per_base_unit: u32,
    pub padding1: u32,
    pub padding2: [u64; 32],
}
