use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1acb5455a11764d6")]
pub struct EvtWithdrawMigrationFeeEvent {
    pub pool: Pubkey,
    pub fee: u64,
    pub flag: u8,
}
