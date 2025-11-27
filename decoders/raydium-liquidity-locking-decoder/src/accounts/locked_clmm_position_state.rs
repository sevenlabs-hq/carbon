use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x34170507aa5a6cd5")]
pub struct LockedClmmPositionState {
    pub bump: [u8; 1],
    pub position_owner: solana_pubkey::Pubkey,
    pub pool_id: solana_pubkey::Pubkey,
    pub position_id: solana_pubkey::Pubkey,
    pub locked_nft_account: solana_pubkey::Pubkey,
    pub fee_nft_mint: solana_pubkey::Pubkey,
    pub recent_epoch: u64,
    pub padding: [u64; 8],
}
