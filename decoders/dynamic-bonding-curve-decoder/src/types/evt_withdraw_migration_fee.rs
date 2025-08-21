use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtWithdrawMigrationFee {
    pub pool: solana_pubkey::Pubkey,
    pub fee: u64,
    pub flag: u8,
}
