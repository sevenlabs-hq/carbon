use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d277330ccf62f4239")]
pub struct UpdatePositionOperator {
    pub position: solana_sdk::pubkey::Pubkey,
    pub old_operator: solana_sdk::pubkey::Pubkey,
    pub new_operator: solana_sdk::pubkey::Pubkey,
}
