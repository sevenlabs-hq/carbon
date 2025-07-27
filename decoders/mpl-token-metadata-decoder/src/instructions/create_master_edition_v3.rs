use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11")]
pub struct CreateMasterEditionV3 {
    pub create_master_edition_args: CreateMasterEditionArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateMasterEditionV3InstructionAccounts {
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMasterEditionV3 {
    type ArrangedAccounts = CreateMasterEditionV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let edition = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let mint_authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter);

        Some(CreateMasterEditionV3InstructionAccounts {
            edition,
            mint,
            update_authority,
            mint_authority,
            payer,
            metadata,
            token_program,
            system_program,
            rent,
        })
    }
}
