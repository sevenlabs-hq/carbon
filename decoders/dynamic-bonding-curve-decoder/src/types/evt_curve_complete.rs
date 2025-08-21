

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EvtCurveComplete {
    pub pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
}
