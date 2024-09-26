
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x434ec469d3193efc")]
pub struct IncreaseLiquidityV2{
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
    pub base_flag: Option<bool>,
}

pub struct IncreaseLiquidityV2InstructionAccounts {
    pub nft_owner: solana_sdk::pubkey::Pubkey,
    pub nft_account: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_position: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub token_account0: solana_sdk::pubkey::Pubkey,
    pub token_account1: solana_sdk::pubkey::Pubkey,
    pub token_vault0: solana_sdk::pubkey::Pubkey,
    pub token_vault1: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub vault0_mint: solana_sdk::pubkey::Pubkey,
    pub vault1_mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for IncreaseLiquidityV2 {
    type ArrangedAccounts = IncreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let nft_owner = accounts.get(0)?;
        let nft_account = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let protocol_position = accounts.get(3)?;
        let personal_position = accounts.get(4)?;
        let tick_array_lower = accounts.get(5)?;
        let tick_array_upper = accounts.get(6)?;
        let token_account0 = accounts.get(7)?;
        let token_account1 = accounts.get(8)?;
        let token_vault0 = accounts.get(9)?;
        let token_vault1 = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let token_program2022 = accounts.get(12)?;
        let vault0_mint = accounts.get(13)?;
        let vault1_mint = accounts.get(14)?;

        Some(IncreaseLiquidityV2InstructionAccounts {
            nft_owner: *nft_owner,
            nft_account: *nft_account,
            pool_state: *pool_state,
            protocol_position: *protocol_position,
            personal_position: *personal_position,
            tick_array_lower: *tick_array_lower,
            tick_array_upper: *tick_array_upper,
            token_account0: *token_account0,
            token_account1: *token_account1,
            token_vault0: *token_vault0,
            token_vault1: *token_vault1,
            token_program: *token_program,
            token_program2022: *token_program2022,
            vault0_mint: *vault0_mint,
            vault1_mint: *vault1_mint,
        })
    }
}