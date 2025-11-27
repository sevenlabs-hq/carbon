use carbon_core::{CarbonDeserialize, borsh};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x8628b74f0c70a235")]
pub struct SupportMintAssociated {
    pub bump: u8,
    pub mint: solana_pubkey::Pubkey,
    pub padding: [u64; 8],
}
