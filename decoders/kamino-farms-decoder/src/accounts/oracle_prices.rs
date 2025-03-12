
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x598076dd0648b492")] 
pub struct OraclePrices { 
        pub oracle_mappings: solana_sdk::pubkey::Pubkey, 
        pub prices: [DatedPrice; 512], 
}