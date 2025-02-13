use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d74f461e8671f983a")]
pub struct RemoveLiquidityEvent {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub from: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub amounts: [u64; 2],
    pub active_bin_id: i32,
}
