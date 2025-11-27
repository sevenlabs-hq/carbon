use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9dd6dceb6287ab1c")]
pub struct UserMetadata {
    pub referrer: solana_pubkey::Pubkey,
    pub bump: u64,
    pub user_lookup_table: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding1: [u64; 51],
    #[serde(with = "serde_big_array::BigArray")]
    pub padding2: [u64; 64],
}
