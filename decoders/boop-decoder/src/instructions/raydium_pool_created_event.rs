use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1daab215d754de2265")]
pub struct RaydiumPoolCreatedEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
}
