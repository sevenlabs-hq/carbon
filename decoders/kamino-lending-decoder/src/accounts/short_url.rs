use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1c59ae19e27c7ed4")]
pub struct ShortUrl {
    pub referrer: solana_pubkey::Pubkey,
    pub short_url: String,
}
