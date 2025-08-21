use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

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
        let mut iter = accounts.iter();
        let virtual_pool = next_account(&mut iter)?;
        let virtual_pool_metadata = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CreateVirtualPoolMetadataInstructionAccounts {
            virtual_pool,
            virtual_pool_metadata,
            creator,
            payer,
            system_program,
            event_authority,
            program,
        })
    }
}
