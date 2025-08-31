use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0xd040d1cc71e21662")]
pub struct ProtocolOwnerState {
    pub current_protocol_owner: solana_pubkey::Pubkey,
}
