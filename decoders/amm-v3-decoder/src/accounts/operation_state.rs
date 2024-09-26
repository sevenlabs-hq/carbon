 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use crate::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x13ec3aed51deb7fc")] 
pub struct OperationState { 
        pub bump: u8, 
        pub operation_owners: [solana_sdk::pubkey::Pubkey; 10], 
        pub whitelist_mints: [solana_sdk::pubkey::Pubkey; 100], 
}