use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x13")]
pub struct Utilize {
    pub utilize_args: UtilizeArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UtilizeInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub use_authority: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub use_authority_record: Option<solana_pubkey::Pubkey>,
    pub burner: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for Utilize {
    type ArrangedAccounts = UtilizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let token_account = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let use_authority = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let use_authority_record = next_account(&mut iter);
        let burner = next_account(&mut iter);

        Some(UtilizeInstructionAccounts {
            metadata,
            token_account,
            mint,
            use_authority,
            owner,
            token_program,
            ata_program,
            system_program,
            rent,
            use_authority_record,
            burner,
        })
    }
}
