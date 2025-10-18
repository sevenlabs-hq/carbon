use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db5697f4308bb7839")]
pub struct EvtPartnerWithdrawMigrationFeeEvent {
    pub pool: Pubkey,
    pub fee: u64,
}
