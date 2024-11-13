 
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x1bbb206489fd68f2")] 
pub struct TokenRecord { 
        pub key: Key, 
        pub bump: u8, 
        pub state: TokenState, 
        pub rule_set_revision: Option<u64>, 
        pub delegate: Option<solana_sdk::pubkey::Pubkey>, 
        pub delegate_role: Option<TokenDelegateRole>, 
        pub locked_transfer: Option<solana_sdk::pubkey::Pubkey>, 
}
