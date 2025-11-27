use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x34")]
pub struct Verify {
    pub verification_args: VerificationArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct VerifyInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub delegate_record: Option<solana_pubkey::Pubkey>,
    pub metadata: solana_pubkey::Pubkey,
    pub collection_mint: Option<solana_pubkey::Pubkey>,
    pub collection_metadata: Option<solana_pubkey::Pubkey>,
    pub collection_master_edition: Option<solana_pubkey::Pubkey>,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Verify {
    type ArrangedAccounts = VerifyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let delegate_record = next_account(&mut iter);
        let metadata = next_account(&mut iter)?;
        let collection_mint = next_account(&mut iter);
        let collection_metadata = next_account(&mut iter);
        let collection_master_edition = next_account(&mut iter);
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;

        Some(VerifyInstructionAccounts {
            authority,
            delegate_record,
            metadata,
            collection_mint,
            collection_metadata,
            collection_master_edition,
            system_program,
            sysvar_instructions,
        })
    }
}
