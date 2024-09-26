
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd2121b7a10de4e44")]
pub struct CollectProtocolFee{
    pub amount0_requested: u64,
    pub amount1_requested: u64,
}

pub struct CollectProtocolFeeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub token_vault0: solana_sdk::pubkey::Pubkey,
    pub token_vault1: solana_sdk::pubkey::Pubkey,
    pub vault0_mint: solana_sdk::pubkey::Pubkey,
    pub vault1_mint: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account0: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account1: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CollectProtocolFee {
    type ArrangedAccounts = CollectProtocolFeeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let pool_state = accounts.get(1)?;
        let amm_config = accounts.get(2)?;
        let token_vault0 = accounts.get(3)?;
        let token_vault1 = accounts.get(4)?;
        let vault0_mint = accounts.get(5)?;
        let vault1_mint = accounts.get(6)?;
        let recipient_token_account0 = accounts.get(7)?;
        let recipient_token_account1 = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let token_program2022 = accounts.get(10)?;

        Some(CollectProtocolFeeInstructionAccounts {
            owner: *owner,
            pool_state: *pool_state,
            amm_config: *amm_config,
            token_vault0: *token_vault0,
            token_vault1: *token_vault1,
            vault0_mint: *vault0_mint,
            vault1_mint: *vault1_mint,
            recipient_token_account0: *recipient_token_account0,
            recipient_token_account1: *recipient_token_account1,
            token_program: *token_program,
            token_program2022: *token_program2022,
        })
    }
}