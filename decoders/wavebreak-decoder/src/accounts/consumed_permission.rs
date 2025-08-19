use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct ConsumedPermission {
    pub discriminator: AccountDiscriminator,
    pub padding1: [u8; 7],
    pub safe_to_close_slot: u64,
    pub refund_destination: solana_pubkey::Pubkey,
    pub padding2: [u8; 16],
}
