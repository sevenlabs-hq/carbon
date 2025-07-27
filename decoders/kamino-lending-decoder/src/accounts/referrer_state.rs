use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc251d9670c130c42")]
pub struct ReferrerState {
    pub short_url: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}
