
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x856e4aaf709ff59f")]
pub struct InitializeOrder{
    pub params: InitializeOrderParams,
}

pub struct InitializeOrderInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub input_mint_reserve: solana_sdk::pubkey::Pubkey,
    pub maker_input_mint_account: solana_sdk::pubkey::Pubkey,
    pub fee: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub output_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeOrder {
    type ArrangedAccounts = InitializeOrderInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let maker = accounts.get(1)?;
        let order = accounts.get(2)?;
        let input_mint_reserve = accounts.get(3)?;
        let maker_input_mint_account = accounts.get(4)?;
        let fee = accounts.get(5)?;
        let referral = accounts.get(6)?;
        let input_mint = accounts.get(7)?;
        let output_mint = accounts.get(8)?;
        let input_token_program = accounts.get(9)?;
        let output_token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let event_authority = accounts.get(13)?;
        let program = accounts.get(14)?;

        Some(InitializeOrderInstructionAccounts {
            payer: *payer,
            maker: *maker,
            order: *order,
            input_mint_reserve: *input_mint_reserve,
            maker_input_mint_account: *maker_input_mint_account,
            fee: *fee,
            referral: *referral,
            input_mint: *input_mint,
            output_mint: *output_mint,
            input_token_program: *input_token_program,
            output_token_program: *output_token_program,
            system_program: *system_program,
            associated_token_program: *associated_token_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}