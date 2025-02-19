use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub init_amount0: u64,
    pub init_amount1: u64,
    pub open_time: u64,
}

pub struct InitializeInstructionAccounts {
    pub creator: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub token0_mint: solana_sdk::pubkey::Pubkey,
    pub token1_mint: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub creator_token0: solana_sdk::pubkey::Pubkey,
    pub creator_token1: solana_sdk::pubkey::Pubkey,
    pub creator_lp_token: solana_sdk::pubkey::Pubkey,
    pub token0_vault: solana_sdk::pubkey::Pubkey,
    pub token1_vault: solana_sdk::pubkey::Pubkey,
    pub create_pool_fee: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token0_program: solana_sdk::pubkey::Pubkey,
    pub token1_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [creator, amm_config, authority, pool_state, token0_mint, token1_mint, lp_mint, creator_token0, creator_token1, creator_lp_token, token0_vault, token1_vault, create_pool_fee, observation_state, token_program, token0_program, token1_program, associated_token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            creator: creator.pubkey,
            amm_config: amm_config.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            token0_mint: token0_mint.pubkey,
            token1_mint: token1_mint.pubkey,
            lp_mint: lp_mint.pubkey,
            creator_token0: creator_token0.pubkey,
            creator_token1: creator_token1.pubkey,
            creator_lp_token: creator_lp_token.pubkey,
            token0_vault: token0_vault.pubkey,
            token1_vault: token1_vault.pubkey,
            create_pool_fee: create_pool_fee.pubkey,
            observation_state: observation_state.pubkey,
            token_program: token_program.pubkey,
            token0_program: token0_program.pubkey,
            token1_program: token1_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
