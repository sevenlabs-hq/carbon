use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x68dddbcb0a8efaa3")]
pub struct MeteoraDammV2Metadata {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub pool_creator: solana_pubkey::Pubkey,
    pub partner: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 126],
}
