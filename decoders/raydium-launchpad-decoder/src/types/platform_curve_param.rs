use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlatformCurveParam {
    pub epoch: u64,
    pub index: u8,
    pub global_config: solana_pubkey::Pubkey,
    pub bonding_curve_param: BondingCurveParam,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u64; 50],
}
