use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OverrideExpiryArgs {
    pub expiry_index: u8,
    pub active_ts: u64,
    pub expiry_ts: u64,
}
