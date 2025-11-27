use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9cf709bc366c554d")]
pub struct TokenLedger {
    pub token_account: solana_pubkey::Pubkey,
    pub amount: u64,
}
