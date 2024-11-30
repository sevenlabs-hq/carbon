

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum Authority {
    None,
    Owner,
    UpdateAuthority,
    Address
                {
                    address: solana_sdk::pubkey::Pubkey,
                }
    ,
}


