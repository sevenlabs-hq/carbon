use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb16b25b4a01331d1")]
pub struct CollectRewardV2 {
    pub reward_index: u8,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectRewardV2InstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub reward_owner_account: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub reward_token_program: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectRewardV2 {
    type ArrangedAccounts = CollectRewardV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let position_authority = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let reward_owner_account = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_token_program = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;

        Some(CollectRewardV2InstructionAccounts {
            whirlpool,
            position_authority,
            position,
            position_token_account,
            reward_owner_account,
            reward_mint,
            reward_vault,
            reward_token_program,
            memo_program,
        })
    }
}
