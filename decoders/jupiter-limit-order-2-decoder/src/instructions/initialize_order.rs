
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;


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

impl carbon_core::deserialize::ArrangeAccounts for InitializeOrder {
    type ArrangedAccounts = InitializeOrderInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
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
            payer: payer.pubkey,
            maker: maker.pubkey,
            order: order.pubkey,
            input_mint_reserve: input_mint_reserve.pubkey,
            maker_input_mint_account: maker_input_mint_account.pubkey,
            fee: fee.pubkey,
            referral: referral.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
