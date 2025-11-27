use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SupportMintAssociated {
    pub bump: u8,
    pub mint: solana_pubkey::Pubkey,
    pub padding: [u64; 8],
}
