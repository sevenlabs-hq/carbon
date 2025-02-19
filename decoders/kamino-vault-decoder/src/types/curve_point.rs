use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CurvePoint {
    pub utilization_rate_bps: u32,
    pub borrow_rate_bps: u32,
}
