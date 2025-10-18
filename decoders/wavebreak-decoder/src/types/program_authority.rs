use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ProgramAuthority {
    pub key: solana_pubkey::Pubkey,
    pub privileges: [u8; 32],
}
