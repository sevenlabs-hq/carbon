 
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xe90a12e681ac25ea")] 
pub struct EditionMarker { 
        pub key: Key, 
        pub ledger: [u8; 31], 
}
