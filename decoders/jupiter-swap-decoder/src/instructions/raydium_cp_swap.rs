use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x36ea538d34bf2e90")]
pub struct RaydiumCpSwap {}

pub struct RaydiumCpSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub output_vault: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub output_token_program: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub output_token_mint: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RaydiumCpSwap {
    type ArrangedAccounts = RaydiumCpSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let amm_config = accounts.get(3)?;
        let pool_state = accounts.get(4)?;
        let input_token_account = accounts.get(5)?;
        let output_token_account = accounts.get(6)?;
        let input_vault = accounts.get(7)?;
        let output_vault = accounts.get(8)?;
        let input_token_program = accounts.get(9)?;
        let output_token_program = accounts.get(10)?;
        let input_token_mint = accounts.get(11)?;
        let output_token_mint = accounts.get(12)?;
        let observation_state = accounts.get(13)?;

        Some(RaydiumCpSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            input_vault: input_vault.pubkey,
            output_vault: output_vault.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
            input_token_mint: input_token_mint.pubkey,
            output_token_mint: output_token_mint.pubkey,
            observation_state: observation_state.pubkey,
        })
    }
}
