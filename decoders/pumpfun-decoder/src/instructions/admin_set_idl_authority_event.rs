use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df53b46224bb96d5c")]
pub struct AdminSetIdlAuthorityEvent {
    pub idl_authority: solana_pubkey::Pubkey,
}
