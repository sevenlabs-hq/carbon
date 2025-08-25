use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MigratedPoolFee {
    pub collect_fee_mode: u8,
    pub dynamic_fee: u8,
    pub pool_fee_bps: u16,
}
