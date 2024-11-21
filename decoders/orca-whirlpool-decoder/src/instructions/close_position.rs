use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7b86510031446262")]
pub struct ClosePosition {}

pub struct ClosePositionInstructionAccounts {
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePosition {
    type ArrangedAccounts = ClosePositionInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let position_authority = accounts.get(0)?;
        let receiver = accounts.get(1)?;
        let position = accounts.get(2)?;
        let position_mint = accounts.get(3)?;
        let position_token_account = accounts.get(4)?;
        let token_program = accounts.get(5)?;

        Some(ClosePositionInstructionAccounts {
            position_authority: position_authority.pubkey,
            receiver: receiver.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_token_account: position_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
