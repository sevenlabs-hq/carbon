use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x1c59ae19e27c7ed4")]
pub struct ShortUrl {
    pub referrer: solana_pubkey::Pubkey,
    pub short_url: String,
}
