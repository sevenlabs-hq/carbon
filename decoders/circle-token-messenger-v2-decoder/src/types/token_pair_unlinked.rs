use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenPairUnlinked {
    pub local_token: solana_pubkey::Pubkey,
    pub remote_domain: u32,
    pub remote_token: solana_pubkey::Pubkey,
}
