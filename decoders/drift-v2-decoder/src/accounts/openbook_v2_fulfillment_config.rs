use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x032b3a6a8384c7ab")]
pub struct OpenbookV2FulfillmentConfig {
    pub pubkey: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_program_id: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_market: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_market_authority: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_event_heap: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_bids: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_asks: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_base_vault: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_quote_vault: solana_sdk::pubkey::Pubkey,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
