use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtPartnerWithdrawMigrationFee {
    pub pool: solana_pubkey::Pubkey,
    pub fee: u64,
}
