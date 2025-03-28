use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df714a5f84b0536f6")]
pub struct OverrideCurveParamEvent {
    pub new_amp: u64,
    pub updated_timestamp: u64,
    pub pool: solana_sdk::pubkey::Pubkey,
}
