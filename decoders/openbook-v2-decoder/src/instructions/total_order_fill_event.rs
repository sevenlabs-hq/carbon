use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d08eb303aae4c9c69")]
pub struct TotalOrderFillEvent {
    pub side: u8,
    pub taker: solana_pubkey::Pubkey,
    pub total_quantity_paid: u64,
    pub total_quantity_received: u64,
    pub fees: u64,
}
