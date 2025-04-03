use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d17d210e962f55952")]
pub struct DecreasePositionPostSwapEvent {
    pub position_request_key: solana_pubkey::Pubkey,
    pub swap_amount: u64,
    pub jupiter_minimum_out: Option<u64>,
}
