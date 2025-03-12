
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xb617adf097ceb643")] 
pub struct MarginfiGroup { 
        pub admin: solana_sdk::pubkey::Pubkey, 
        pub padding0: [u128; 32], 
        pub padding1: [u128; 32], 
}