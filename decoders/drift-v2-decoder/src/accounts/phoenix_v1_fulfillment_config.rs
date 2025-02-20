use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe92d3e2823813048")]
pub struct PhoenixV1FulfillmentConfig {
    pub pubkey: solana_sdk::pubkey::Pubkey,
    pub phoenix_program_id: solana_sdk::pubkey::Pubkey,
    pub phoenix_log_authority: solana_sdk::pubkey::Pubkey,
    pub phoenix_market: solana_sdk::pubkey::Pubkey,
    pub phoenix_base_vault: solana_sdk::pubkey::Pubkey,
    pub phoenix_quote_vault: solana_sdk::pubkey::Pubkey,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
