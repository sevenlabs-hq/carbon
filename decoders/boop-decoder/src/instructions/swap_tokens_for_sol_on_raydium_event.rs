use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4cf9dda241467620")]
pub struct SwapTokensForSolOnRaydiumEvent {
    pub mint: solana_pubkey::Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}
