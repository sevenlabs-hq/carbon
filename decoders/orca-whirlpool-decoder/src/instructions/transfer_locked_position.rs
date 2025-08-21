use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb379e52e438ac28a")]
pub struct TransferLockedPosition {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferLockedPositionInstructionAccounts {
    pub position_authority: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub lock_config: solana_pubkey::Pubkey,
    pub token2022_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferLockedPosition {
    type ArrangedAccounts = TransferLockedPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let position_authority = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_mint = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let destination_token_account = next_account(&mut iter)?;
        let lock_config = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;

        Some(TransferLockedPositionInstructionAccounts {
            position_authority,
            receiver,
            position,
            position_mint,
            position_token_account,
            destination_token_account,
            lock_config,
            token2022_program,
        })
    }
}
