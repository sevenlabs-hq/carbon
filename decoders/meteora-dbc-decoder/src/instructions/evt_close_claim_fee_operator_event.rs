use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6f2725376ed8c217")]
pub struct EvtCloseClaimFeeOperatorEvent {
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
}
