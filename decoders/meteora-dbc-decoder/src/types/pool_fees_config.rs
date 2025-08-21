use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeConfig,
    pub dynamic_fee: DynamicFeeConfig,
    pub padding_0: [u64; 5],
    pub padding_1: [u8; 6],
    pub protocol_fee_percent: u8,
    pub referral_fee_percent: u8,
}
