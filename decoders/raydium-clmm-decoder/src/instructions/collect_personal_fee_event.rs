use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da6ae69c051a15369")]
pub struct CollectPersonalFeeEvent {
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account0: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account1: solana_sdk::pubkey::Pubkey,
    pub amount0: u64,
    pub amount1: u64,
}
