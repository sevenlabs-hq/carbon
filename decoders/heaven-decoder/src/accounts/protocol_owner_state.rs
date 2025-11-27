use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd040d1cc71e21662")]
pub struct ProtocolOwnerState {
    pub current_protocol_owner: solana_pubkey::Pubkey,
}
