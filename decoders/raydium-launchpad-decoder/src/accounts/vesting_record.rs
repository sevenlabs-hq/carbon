use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6af3ddcde67e5553")]
pub struct VestingRecord {
    pub epoch: u64,
    pub pool: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub claimed_amount: u64,
    pub token_share_amount: u64,
    pub padding: [u64; 8],
}
