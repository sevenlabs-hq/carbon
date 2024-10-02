
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x58b746f9d67652d2")]
pub struct SerumSwap{
}

pub struct SerumSwapInstructionAccounts {
    pub market: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub order_payer_token_account: solana_sdk::pubkey::Pubkey,
    pub coin_wallet: solana_sdk::pubkey::Pubkey,
    pub pc_wallet: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SerumSwap {
    type ArrangedAccounts = SerumSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let market = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let order_payer_token_account = accounts.get(2)?;
        let coin_wallet = accounts.get(3)?;
        let pc_wallet = accounts.get(4)?;
        let dex_program = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let rent = accounts.get(7)?;

        Some(SerumSwapInstructionAccounts {
            market: *market,
            authority: *authority,
            order_payer_token_account: *order_payer_token_account,
            coin_wallet: *coin_wallet,
            pc_wallet: *pc_wallet,
            dex_program: *dex_program,
            token_program: *token_program,
            rent: *rent,
        })
    }
}