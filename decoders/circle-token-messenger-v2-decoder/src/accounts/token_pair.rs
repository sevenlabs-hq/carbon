use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11d62db0e595c547")]
pub struct TokenPair {
    pub remote_domain: u32,
    pub remote_token: solana_pubkey::Pubkey,
    pub local_token: solana_pubkey::Pubkey,
    pub bump: u8,
}
