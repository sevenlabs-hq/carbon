use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateZetaPricingPubkeysArgs {
    pub asset: Asset,
    pub oracle: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub perp_sync_queue: solana_pubkey::Pubkey,
    pub zeta_group_key: solana_pubkey::Pubkey,
    pub reset_oracle: bool,
}
