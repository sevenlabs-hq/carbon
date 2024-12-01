
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum LinkedDataKey {
    LinkedLifecycleHook
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
    LinkedAppData
                (
                    Authority,
                )
    ,
}


