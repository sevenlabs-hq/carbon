use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x74d2bb77c4c43489")]
pub struct PoolAccount {
    pub pool_header: PoolHeader,
    pub amm: Amm,
}
