use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum UpdateGlobalConfigValue {
    Bool(bool),
    U16(u16),
    U64(u64),
    Pubkey(solana_pubkey::Pubkey),
}
