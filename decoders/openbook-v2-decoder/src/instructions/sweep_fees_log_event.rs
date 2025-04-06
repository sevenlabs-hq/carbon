use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd2f21a4d5e30ff3d")]
pub struct SweepFeesLogEvent {
    pub market: solana_pubkey::Pubkey,
    pub amount: u64,
    pub receiver: solana_pubkey::Pubkey,
}
