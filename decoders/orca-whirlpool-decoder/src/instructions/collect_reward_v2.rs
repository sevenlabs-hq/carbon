use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb16b25b4a01331d1")]
pub struct CollectRewardV2 {
    pub reward_index: u8,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

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
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, position_authority, position, position_token_account, reward_owner_account, reward_mint, reward_vault, reward_token_program, memo_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectRewardV2InstructionAccounts {
            whirlpool: whirlpool.pubkey,
            position_authority: position_authority.pubkey,
            position: position.pubkey,
            position_token_account: position_token_account.pubkey,
            reward_owner_account: reward_owner_account.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_token_program: reward_token_program.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
