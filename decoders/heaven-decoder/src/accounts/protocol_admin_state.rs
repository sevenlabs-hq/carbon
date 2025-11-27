use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x187caee1e81e73c0")]
pub struct ProtocolAdminState {
    pub current_protocol_admin: solana_pubkey::Pubkey,
}
