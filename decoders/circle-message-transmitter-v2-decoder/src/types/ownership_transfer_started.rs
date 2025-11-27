use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OwnershipTransferStarted {
    pub previous_owner: solana_pubkey::Pubkey,
    pub new_owner: solana_pubkey::Pubkey,
}
