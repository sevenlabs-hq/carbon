use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14")]
pub struct ApproveUseAuthority {
    pub approve_use_authority_args: ApproveUseAuthorityArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ApproveUseAuthorityInstructionAccounts {
    pub use_authority_record: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub owner_token_account: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub burner: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for ApproveUseAuthority {
    type ArrangedAccounts = ApproveUseAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let use_authority_record = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let owner_token_account = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let burner = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter);

        Some(ApproveUseAuthorityInstructionAccounts {
            use_authority_record,
            owner,
            payer,
            user,
            owner_token_account,
            metadata,
            mint,
            burner,
            token_program,
            system_program,
            rent,
        })
    }
}
