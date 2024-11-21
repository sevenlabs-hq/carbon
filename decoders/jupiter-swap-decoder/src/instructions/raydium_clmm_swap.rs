use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2fb8d5c123d25704")]
pub struct RaydiumClmmSwap {}

pub struct RaydiumClmmSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub output_vault: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub tick_array: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RaydiumClmmSwap {
    type ArrangedAccounts = RaydiumClmmSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let amm_config = accounts.get(2)?;
        let pool_state = accounts.get(3)?;
        let input_token_account = accounts.get(4)?;
        let output_token_account = accounts.get(5)?;
        let input_vault = accounts.get(6)?;
        let output_vault = accounts.get(7)?;
        let observation_state = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let tick_array = accounts.get(10)?;

        Some(RaydiumClmmSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            payer: payer.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            input_vault: input_vault.pubkey,
            output_vault: output_vault.pubkey,
            observation_state: observation_state.pubkey,
            token_program: token_program.pubkey,
            tick_array: tick_array.pubkey,
        })
    }
}
