 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use crate::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9cf709bc366c554d")] 
pub struct TokenLedger { 
        pub token_account: solana_sdk::pubkey::Pubkey, 
        pub amount: u64, 
}