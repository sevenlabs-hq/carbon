use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x190aeec5cfea4916")]
pub struct LockedCpLiquidityState {
    pub locked_lp_amount: u64,
    pub claimed_lp_amount: u64,
    pub unclaimed_lp_amount: u64,
    pub last_lp: u64,
    pub last_k: u128,
    pub recent_epoch: u64,
    pub pool_id: solana_pubkey::Pubkey,
    pub fee_nft_mint: solana_pubkey::Pubkey,
    pub locked_owner: solana_pubkey::Pubkey,
    pub locked_lp_mint: solana_pubkey::Pubkey,
    pub padding: [u64; 8],
}
