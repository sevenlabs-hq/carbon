use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x12eda6c52210d590")]
pub struct CollectRemainingRewards {
    pub reward_index: u8,
}

pub struct CollectRemainingRewardsInstructionAccounts {
    pub reward_funder: solana_sdk::pubkey::Pubkey,
    pub funder_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub reward_token_vault: solana_sdk::pubkey::Pubkey,
    pub reward_vault_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectRemainingRewards {
    type ArrangedAccounts = CollectRemainingRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let reward_funder = accounts.get(0)?;
        let funder_token_account = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let reward_token_vault = accounts.get(3)?;
        let reward_vault_mint = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let token_program2022 = accounts.get(6)?;
        let memo_program = accounts.get(7)?;

        Some(CollectRemainingRewardsInstructionAccounts {
            reward_funder: reward_funder.pubkey,
            funder_token_account: funder_token_account.pubkey,
            pool_state: pool_state.pubkey,
            reward_token_vault: reward_token_vault.pubkey,
            reward_vault_mint: reward_vault_mint.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
