use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AdminSetIdlAuthorityEvent {
    pub idl_authority: solana_pubkey::Pubkey,
}
