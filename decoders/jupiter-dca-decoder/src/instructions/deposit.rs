use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3ecdf2aff4a98834")]
pub struct Deposit {
    pub dca_key: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
}
