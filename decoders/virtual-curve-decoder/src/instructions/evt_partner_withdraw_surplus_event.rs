use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc3389809e8482316")]
pub struct EvtPartnerWithdrawSurplusEvent {
    pub pool: solana_pubkey::Pubkey,
    pub surplus_amount: u64,
}
