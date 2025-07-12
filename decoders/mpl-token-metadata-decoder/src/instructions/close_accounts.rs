use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x39")]
pub struct CloseAccounts {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseAccountsInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseAccounts {
    type ArrangedAccounts = CloseAccountsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let destination = next_account(&mut iter)?;

        Some(CloseAccountsInstructionAccounts {
            metadata,
            edition,
            mint,
            authority,
            destination,
        })
    }
}
