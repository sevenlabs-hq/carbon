use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xcfc24e8a9e4aba7f")]
pub struct ReferrerIdAccount {
    pub referrer_id: [u8; 6],
    pub referrer_pubkey: solana_pubkey::Pubkey,
}
