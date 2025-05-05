use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AuthorityTransferInitiatedEvent {
    pub old_authority: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
}
