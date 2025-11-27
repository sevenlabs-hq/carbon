use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct VerifiedCreatorsSignature {
    pub address: solana_pubkey::Pubkey,
    pub verified: bool,
}
