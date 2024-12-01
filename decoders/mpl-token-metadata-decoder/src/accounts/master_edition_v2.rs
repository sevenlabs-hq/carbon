use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct MasterEditionV2 {
    pub key: Key,
    pub supply: u64,
    pub max_supply: Option<u64>,
}
