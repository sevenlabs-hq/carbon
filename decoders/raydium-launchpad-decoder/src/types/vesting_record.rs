use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct VestingRecord {
    pub epoch: u64,
    pub pool: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub claimed_amount: u64,
    pub token_share_amount: u64,
    pub padding: [u64; 8],
}
