use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4759de7cd7c0e68a")]
pub struct TokenBoughtEvent {
    pub mint: solana_pubkey::Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_fee: u64,
    pub buyer: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
}
