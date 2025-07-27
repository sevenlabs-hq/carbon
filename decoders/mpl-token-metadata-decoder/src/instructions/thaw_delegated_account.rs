use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct ThawDelegatedAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ThawDelegatedAccountInstructionAccounts {
    pub delegate: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ThawDelegatedAccount {
    type ArrangedAccounts = ThawDelegatedAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let delegate = next_account(&mut iter)?;
        let token_account = next_account(&mut iter)?;
        let edition = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(ThawDelegatedAccountInstructionAccounts {
            delegate,
            token_account,
            edition,
            mint,
            token_program,
        })
    }
}
