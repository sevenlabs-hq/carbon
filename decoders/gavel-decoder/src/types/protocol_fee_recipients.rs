use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ProtocolFeeRecipients {
    pub recipients: [ProtocolFeeRecipient; 3],
    pub padding: [u64; 12],
}
