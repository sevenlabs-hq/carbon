use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PermissionMessage {
    V1 {
        nonce: u64,
        consumer_program: solana_pubkey::Pubkey,
        permission_signer: PermissionSigner,
        permission_subject: solana_pubkey::Pubkey,
        valid_until: u64,
        permission_type: u8,
        instruction_discriminators: Vec<Vec<u8>>,
    },
}
