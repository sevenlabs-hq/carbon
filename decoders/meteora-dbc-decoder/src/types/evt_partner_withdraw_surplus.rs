use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtPartnerWithdrawSurplus {
    pub pool: solana_pubkey::Pubkey,
    pub surplus_amount: u64,
}
