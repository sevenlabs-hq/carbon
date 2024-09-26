
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb179ad7e886ca3f0")]
pub struct DecreaseLiquidity{
    pub liquidity: u128,
    pub amount0_min: u64,
    pub amount1_min: u64,
}

pub struct DecreaseLiquidityInstructionAccounts {
    pub nft_owner: solana_sdk::pubkey::Pubkey,
    pub nft_account: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_position: solana_sdk::pubkey::Pubkey,
    pub token_vault0: solana_sdk::pubkey::Pubkey,
    pub token_vault1: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account0: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account1: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DecreaseLiquidity {
    type ArrangedAccounts = DecreaseLiquidityInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let nft_owner = accounts.get(0)?;
        let nft_account = accounts.get(1)?;
        let personal_position = accounts.get(2)?;
        let pool_state = accounts.get(3)?;
        let protocol_position = accounts.get(4)?;
        let token_vault0 = accounts.get(5)?;
        let token_vault1 = accounts.get(6)?;
        let tick_array_lower = accounts.get(7)?;
        let tick_array_upper = accounts.get(8)?;
        let recipient_token_account0 = accounts.get(9)?;
        let recipient_token_account1 = accounts.get(10)?;
        let token_program = accounts.get(11)?;

        Some(DecreaseLiquidityInstructionAccounts {
            nft_owner: *nft_owner,
            nft_account: *nft_account,
            personal_position: *personal_position,
            pool_state: *pool_state,
            protocol_position: *protocol_position,
            token_vault0: *token_vault0,
            token_vault1: *token_vault1,
            tick_array_lower: *tick_array_lower,
            tick_array_upper: *tick_array_upper,
            recipient_token_account0: *recipient_token_account0,
            recipient_token_account1: *recipient_token_account1,
            token_program: *token_program,
        })
    }
}