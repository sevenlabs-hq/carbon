use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8bc283b38cb3e5f4")]
pub struct Oracle {
    pub whirlpool: solana_pubkey::Pubkey,
    pub trade_enable_timestamp: u64,
    pub adaptive_fee_constants: AdaptiveFeeConstants,
    pub adaptive_fee_variables: AdaptiveFeeVariables,
    #[serde(with = "serde_big_array::BigArray")]
    pub reserved: [u8; 128],
}
