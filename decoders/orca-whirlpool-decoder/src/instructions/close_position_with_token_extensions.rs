use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01b6873b9b1963df")]
pub struct ClosePositionWithTokenExtensions {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClosePositionWithTokenExtensionsInstructionAccounts {
    pub position_authority: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub token2022_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePositionWithTokenExtensions {
    type ArrangedAccounts = ClosePositionWithTokenExtensionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let position_authority = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_mint = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;

        Some(ClosePositionWithTokenExtensionsInstructionAccounts {
            position_authority,
            receiver,
            position,
            position_mint,
            position_token_account,
            token2022_program,
        })
    }
}
