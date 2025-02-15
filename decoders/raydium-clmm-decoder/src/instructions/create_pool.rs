use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe992d18ecf6840bc")]
pub struct CreatePool {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

pub struct CreatePoolInstructionAccounts {
    pub pool_creator: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub token_mint0: solana_sdk::pubkey::Pubkey,
    pub token_mint1: solana_sdk::pubkey::Pubkey,
    pub token_vault0: solana_sdk::pubkey::Pubkey,
    pub token_vault1: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
    pub tick_array_bitmap: solana_sdk::pubkey::Pubkey,
    pub token_program0: solana_sdk::pubkey::Pubkey,
    pub token_program1: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePool {
    type ArrangedAccounts = CreatePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let pool_creator = accounts.get(0)?;
        let amm_config = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let token_mint0 = accounts.get(3)?;
        let token_mint1 = accounts.get(4)?;
        let token_vault0 = accounts.get(5)?;
        let token_vault1 = accounts.get(6)?;
        let observation_state = accounts.get(7)?;
        let tick_array_bitmap = accounts.get(8)?;
        let token_program0 = accounts.get(9)?;
        let token_program1 = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let rent = accounts.get(12)?;

        Some(CreatePoolInstructionAccounts {
            pool_creator: pool_creator.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            token_mint0: token_mint0.pubkey,
            token_mint1: token_mint1.pubkey,
            token_vault0: token_vault0.pubkey,
            token_vault1: token_vault1.pubkey,
            observation_state: observation_state.pubkey,
            tick_array_bitmap: tick_array_bitmap.pubkey,
            token_program0: token_program0.pubkey,
            token_program1: token_program1.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
