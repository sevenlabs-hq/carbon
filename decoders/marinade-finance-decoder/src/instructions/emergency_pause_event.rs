

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9ff1c0e81dd03315")]
pub struct EmergencyPauseEvent{
    pub state: solana_sdk::pubkey::Pubkey,
}
