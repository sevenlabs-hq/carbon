use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da384d980f35c5af9")]
pub struct AuthorityTransferCompletedEvent {
    pub old_authority: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
}
