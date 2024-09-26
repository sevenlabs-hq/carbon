
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x72712de2b3ef6ae1")]
pub struct SwapV2{
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

pub struct SwapV2InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub output_vault: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub input_vault_mint: solana_sdk::pubkey::Pubkey,
    pub output_vault_mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SwapV2 {
    type ArrangedAccounts = SwapV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let amm_config = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let input_token_account = accounts.get(3)?;
        let output_token_account = accounts.get(4)?;
        let input_vault = accounts.get(5)?;
        let output_vault = accounts.get(6)?;
        let observation_state = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let token_program2022 = accounts.get(9)?;
        let memo_program = accounts.get(10)?;
        let input_vault_mint = accounts.get(11)?;
        let output_vault_mint = accounts.get(12)?;

        Some(SwapV2InstructionAccounts {
            payer: *payer,
            amm_config: *amm_config,
            pool_state: *pool_state,
            input_token_account: *input_token_account,
            output_token_account: *output_token_account,
            input_vault: *input_vault,
            output_vault: *output_vault,
            observation_state: *observation_state,
            token_program: *token_program,
            token_program2022: *token_program2022,
            memo_program: *memo_program,
            input_vault_mint: *input_vault_mint,
            output_vault_mint: *output_vault_mint,
        })
    }
}