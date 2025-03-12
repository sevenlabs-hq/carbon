

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct List {
    pub account: solana_sdk::pubkey::Pubkey,
    pub item_size: u32,
    pub count: u32,
    pub reserved1: solana_sdk::pubkey::Pubkey,
    pub reserved2: u32,
}
