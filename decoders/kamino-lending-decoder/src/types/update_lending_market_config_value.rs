
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum UpdateLendingMarketConfigValue {
    Bool
                (
                    bool,
                )
    ,
    U8
                (
                    u8,
                )
    ,
    U8Array
                (
                    [u8; 8],
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
    U128
                (
                    u128,
                )
    ,
    Pubkey
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
    ElevationGroup
                (
                    ElevationGroup,
                )
    ,
    Name
                (
                    [u8; 32],
                )
    ,
}


