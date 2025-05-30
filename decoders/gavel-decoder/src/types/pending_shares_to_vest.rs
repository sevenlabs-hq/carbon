use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PendingSharesToVest {
    pub deposit_slot: u64,
    pub lp_shares_to_vest: u64,
}
