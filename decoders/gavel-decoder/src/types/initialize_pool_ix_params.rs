use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializePoolIxParams {
    pub lp_fee_in_bps: u64,
    pub protocol_lp_fee_allocation_in_pct: u64,
    pub fee_recipients_params: [ProtocolFeeRecipientParams; 3],
    pub num_slots_to_vest_lp_shares: Option<u64>,
}
