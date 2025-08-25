use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtPartnerWithdrawMigrationFee {
    pub pool: solana_pubkey::Pubkey,
    pub fee: u64,
}
