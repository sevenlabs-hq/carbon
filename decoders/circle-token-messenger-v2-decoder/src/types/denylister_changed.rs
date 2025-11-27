use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DenylisterChanged {
    pub old_denylister: solana_pubkey::Pubkey,
    pub new_denylister: solana_pubkey::Pubkey,
}
