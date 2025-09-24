use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a4aec976840b7f9")]
pub struct CloseToken {
    pub id: u8,
    pub burn_all: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseTokenInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub wallet: solana_pubkey::Pubkey,
    pub program_authority: solana_pubkey::Pubkey,
    pub program_token_account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseToken {
    type ArrangedAccounts = CloseTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let operator = next_account(&mut iter)?;
        let wallet = next_account(&mut iter)?;
        let program_authority = next_account(&mut iter)?;
        let program_token_account = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CloseTokenInstructionAccounts {
            operator,
            wallet,
            program_authority,
            program_token_account,
            mint,
            token_program,
        })
    }
}
