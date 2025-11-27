use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe33e02fcf70aabb9")]
pub struct LockPosition {
    pub lock_type: LockType,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LockPositionInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub lock_config: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub token2022_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LockPosition {
    type ArrangedAccounts = LockPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let position_authority = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_mint = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let lock_config = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(LockPositionInstructionAccounts {
            funder,
            position_authority,
            position,
            position_mint,
            position_token_account,
            lock_config,
            whirlpool,
            token2022_program,
            system_program,
        })
    }
}
