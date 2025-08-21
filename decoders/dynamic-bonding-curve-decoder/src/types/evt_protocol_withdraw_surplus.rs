

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EvtProtocolWithdrawSurplus {
    pub pool: solana_pubkey::Pubkey,
    pub surplus_amount: u64,
}
