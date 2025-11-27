use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe992d18ecf6840bc")]
pub struct CreatePool {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolInstructionAccounts {
    pub pool_creator: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub token_mint_0: solana_pubkey::Pubkey,
    pub token_mint_1: solana_pubkey::Pubkey,
    pub token_vault_0: solana_pubkey::Pubkey,
    pub token_vault_1: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
    pub tick_array_bitmap: solana_pubkey::Pubkey,
    pub token_program_0: solana_pubkey::Pubkey,
    pub token_program_1: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePool {
    type ArrangedAccounts = CreatePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool_creator, amm_config, pool_state, token_mint_0, token_mint_1, token_vault_0, token_vault_1, observation_state, tick_array_bitmap, token_program_0, token_program_1, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreatePoolInstructionAccounts {
            pool_creator: pool_creator.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            token_mint_0: token_mint_0.pubkey,
            token_mint_1: token_mint_1.pubkey,
            token_vault_0: token_vault_0.pubkey,
            token_vault_1: token_vault_1.pubkey,
            observation_state: observation_state.pubkey,
            tick_array_bitmap: tick_array_bitmap.pubkey,
            token_program_0: token_program_0.pubkey,
            token_program_1: token_program_1.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
