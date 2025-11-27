use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AdminSetIdlAuthorityEvent {
    pub idl_authority: solana_pubkey::Pubkey,
}
