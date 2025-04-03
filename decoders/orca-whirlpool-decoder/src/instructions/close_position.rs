use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7b86510031446262")]
pub struct ClosePosition {}

pub struct ClosePositionInstructionAccounts {
    pub position_authority: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePosition {
    type ArrangedAccounts = ClosePositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position_authority, receiver, position, position_mint, position_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
