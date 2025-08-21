use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa47bb48dc264a0af")]
pub struct ResetPositionRange {
    pub new_tick_lower_index: i32,
    pub new_tick_upper_index: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ResetPositionRangeInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResetPositionRange {
    type ArrangedAccounts = ResetPositionRangeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let position_authority = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(ResetPositionRangeInstructionAccounts {
            funder,
            position_authority,
            whirlpool,
            position,
            position_token_account,
            system_program,
        })
    }
}
