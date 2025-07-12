use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23")]
pub struct SetTokenStandard {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetTokenStandardInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub edition: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenStandard {
    type ArrangedAccounts = SetTokenStandardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let edition = next_account(&mut iter);

        Some(SetTokenStandardInstructionAccounts {
            metadata,
            update_authority,
            mint,
            edition,
        })
    }
}
