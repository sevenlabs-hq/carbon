use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5b014d32ebe58531")]
pub struct InitializeRewardV2 {
    pub reward_index: u8,
}

pub struct InitializeRewardV2InstructionAccounts {
    pub reward_authority: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_token_badge: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeRewardV2 {
    type ArrangedAccounts = InitializeRewardV2InstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let reward_authority = accounts.get(0)?;
        let funder = accounts.get(1)?;
        let whirlpool = accounts.get(2)?;
        let reward_mint = accounts.get(3)?;
        let reward_token_badge = accounts.get(4)?;
        let reward_vault = accounts.get(5)?;
        let reward_token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;

        Some(InitializeRewardV2InstructionAccounts {
            reward_authority: reward_authority.pubkey,
            funder: funder.pubkey,
            whirlpool: whirlpool.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_token_badge: reward_token_badge.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_token_program: reward_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
