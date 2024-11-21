
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf02f99440dbee12a")]
pub struct PreFlashFillOrder{
    pub params: PreFlashFillOrderParams,
}

pub struct PreFlashFillOrderInstructionAccounts {
    pub taker: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub input_mint_reserve: solana_sdk::pubkey::Pubkey,
    pub taker_input_mint_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PreFlashFillOrder {
    type ArrangedAccounts = PreFlashFillOrderInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let taker = accounts.get(0)?;
        let order = accounts.get(1)?;
        let input_mint_reserve = accounts.get(2)?;
        let taker_input_mint_account = accounts.get(3)?;
        let input_mint = accounts.get(4)?;
        let input_token_program = accounts.get(5)?;
        let instruction = accounts.get(6)?;

        Some(PreFlashFillOrderInstructionAccounts {
            taker: taker.pubkey,
            order: order.pubkey,
            input_mint_reserve: input_mint_reserve.pubkey,
            taker_input_mint_account: taker_input_mint_account.pubkey,
            input_mint: input_mint.pubkey,
            input_token_program: input_token_program.pubkey,
            instruction: instruction.pubkey,
        })
    }
}
