use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4605845756ebb122")]
pub struct CollectReward {
    pub reward_index: u8,
}

pub struct CollectRewardInstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_owner_account: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectReward {
    type ArrangedAccounts = CollectRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, position_authority, position, position_token_account, reward_owner_account, reward_vault, token_program] =
            accounts
        else {
            return None;
        };

        Some(CollectRewardInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            position_authority: position_authority.pubkey,
            position: position.pubkey,
            position_token_account: position_token_account.pubkey,
            reward_owner_account: reward_owner_account.pubkey,
            reward_vault: reward_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
