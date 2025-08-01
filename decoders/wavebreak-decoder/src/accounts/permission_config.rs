use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6582b0423e24e65d")]
pub struct PermissionConfig {
    pub discriminator: AccountDiscriminator,
    pub consumer_program: solana_pubkey::Pubkey,
    pub allowed_signers: [PermissionSigner; 3],
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 124],
}
