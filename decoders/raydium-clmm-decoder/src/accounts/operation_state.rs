use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x13ec3aed51deb7fc")]
pub struct OperationState {
    pub bump: u8,
    pub operation_owners: [solana_pubkey::Pubkey; 10],
    #[serde(with = "serde_big_array::BigArray")]
    pub whitelist_mints: [solana_pubkey::Pubkey; 100],
}
