use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcfc24e8a9e4aba7f")]
pub struct ReferrerIdAccount {
    pub referrer_id: [u8; 6],
    pub referrer_pubkey: solana_pubkey::Pubkey,
}
