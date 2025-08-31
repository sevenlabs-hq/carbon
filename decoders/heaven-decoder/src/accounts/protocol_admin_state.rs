use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x187caee1e81e73c0")]
pub struct ProtocolAdminState {
    pub current_protocol_admin: solana_pubkey::Pubkey,
}
