use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward {
    pub reward_index: u8,
}

pub struct InitializeRewardInstructionAccounts {
    pub reward_authority: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let reward_authority = accounts.get(0)?;
        let funder = accounts.get(1)?;
        let whirlpool = accounts.get(2)?;
        let reward_mint = accounts.get(3)?;
        let reward_vault = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;

        Some(InitializeRewardInstructionAccounts {
            reward_authority: reward_authority.pubkey,
            funder: funder.pubkey,
            whirlpool: whirlpool.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_vault: reward_vault.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
