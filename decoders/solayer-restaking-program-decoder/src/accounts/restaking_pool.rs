
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x0c05648f7d5e1ad6")] 
pub struct RestakingPool { 
        pub lst_mint: solana_sdk::pubkey::Pubkey, 
        pub rst_mint: solana_sdk::pubkey::Pubkey, 
        pub bump: u8, 
}