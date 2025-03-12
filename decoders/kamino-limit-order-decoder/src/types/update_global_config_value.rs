

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum UpdateGlobalConfigValue {
    Bool
                (
                    bool,
                )
    ,
    U16
                (
                    u16,
                )
    ,
    U64
                (
                    u64,
                )
    ,
    Pubkey
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
}


