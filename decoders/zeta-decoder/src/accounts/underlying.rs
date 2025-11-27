use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xce80984d70a40d02")]
pub struct Underlying {
    pub mint: solana_pubkey::Pubkey,
}
