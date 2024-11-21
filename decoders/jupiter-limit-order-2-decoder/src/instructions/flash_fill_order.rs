
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfc681286a44e128c")]
pub struct FlashFillOrder{
    pub params: FlashFillOrderParams,
}

pub struct FlashFillOrderInstructionAccounts {
    pub taker: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub input_mint_reserve: solana_sdk::pubkey::Pubkey,
    pub maker_output_mint_account: solana_sdk::pubkey::Pubkey,
    pub taker_output_mint_account: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub output_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FlashFillOrder {
    type ArrangedAccounts = FlashFillOrderInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let taker = accounts.get(0)?;
        let maker = accounts.get(1)?;
        let order = accounts.get(2)?;
        let input_mint_reserve = accounts.get(3)?;
        let maker_output_mint_account = accounts.get(4)?;
        let taker_output_mint_account = accounts.get(5)?;
        let fee_account = accounts.get(6)?;
        let input_token_program = accounts.get(7)?;
        let output_mint = accounts.get(8)?;
        let output_token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;

        Some(FlashFillOrderInstructionAccounts {
            taker: taker.pubkey,
            maker: maker.pubkey,
            order: order.pubkey,
            input_mint_reserve: input_mint_reserve.pubkey,
            maker_output_mint_account: maker_output_mint_account.pubkey,
            taker_output_mint_account: taker_output_mint_account.pubkey,
            fee_account: fee_account.pubkey,
            input_token_program: input_token_program.pubkey,
            output_mint: output_mint.pubkey,
            output_token_program: output_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
