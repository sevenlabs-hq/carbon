use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Lockup {
    pub unix_timestamp: i64,
    pub epoch: u64,
    pub custodian: solana_sdk::pubkey::Pubkey,
}
