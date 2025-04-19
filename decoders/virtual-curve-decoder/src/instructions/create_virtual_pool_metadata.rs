use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2d61bb67fe6d7c86")]
pub struct CreateVirtualPoolMetadata {
    pub metadata: CreateVirtualPoolMetadataParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateVirtualPoolMetadataInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub virtual_pool_metadata: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateVirtualPoolMetadata {
    type ArrangedAccounts = CreateVirtualPoolMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, virtual_pool_metadata, creator, payer, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateVirtualPoolMetadataInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            virtual_pool_metadata: virtual_pool_metadata.pubkey,
            creator: creator.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
