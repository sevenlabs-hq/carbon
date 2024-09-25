
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcbc20cc0e8521c92")]
pub struct PhoenixSwap{
}

pub struct PhoenixSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub log_authority: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trader: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
    pub quote_account: solana_sdk::pubkey::Pubkey,
    pub base_vault: solana_sdk::pubkey::Pubkey,
    pub quote_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for PhoenixSwap {
    type ArrangedAccounts = PhoenixSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let log_authority = accounts.get(1)?;
        let market = accounts.get(2)?;
        let trader = accounts.get(3)?;
        let base_account = accounts.get(4)?;
        let quote_account = accounts.get(5)?;
        let base_vault = accounts.get(6)?;
        let quote_vault = accounts.get(7)?;
        let token_program = accounts.get(8)?;

        Some(PhoenixSwapInstructionAccounts {
            swap_program: *swap_program,
            log_authority: *log_authority,
            market: *market,
            trader: *trader,
            base_account: *base_account,
            quote_account: *quote_account,
            base_vault: *base_vault,
            quote_vault: *quote_vault,
            token_program: *token_program,
        })
    }
}