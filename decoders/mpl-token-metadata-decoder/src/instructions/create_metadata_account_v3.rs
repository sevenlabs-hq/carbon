use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x21")]
pub struct CreateMetadataAccountV3 {
    pub create_metadata_account_args_v3: CreateMetadataAccountArgsV3,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateMetadataAccountV3InstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMetadataAccountV3 {
    type ArrangedAccounts = CreateMetadataAccountV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let mint_authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter);

        Some(CreateMetadataAccountV3InstructionAccounts {
            metadata,
            mint,
            mint_authority,
            payer,
            update_authority,
            system_program,
            rent,
        })
    }
}
