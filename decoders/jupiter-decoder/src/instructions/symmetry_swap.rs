
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1172edea9a0cb974")]
pub struct SymmetrySwap{
}

pub struct SymmetrySwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub fund_state: solana_sdk::pubkey::Pubkey,
    pub pda_account: solana_sdk::pubkey::Pubkey,
    pub pda_from_token_account: solana_sdk::pubkey::Pubkey,
    pub buyer_from_token_account: solana_sdk::pubkey::Pubkey,
    pub pda_to_token_account: solana_sdk::pubkey::Pubkey,
    pub buyer_to_token_account: solana_sdk::pubkey::Pubkey,
    pub swap_fee_account: solana_sdk::pubkey::Pubkey,
    pub host_fee_account: solana_sdk::pubkey::Pubkey,
    pub manager_fee_account: solana_sdk::pubkey::Pubkey,
    pub token_list: solana_sdk::pubkey::Pubkey,
    pub prism_data: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SymmetrySwap {
    type ArrangedAccounts = SymmetrySwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let buyer = accounts.get(1)?;
        let fund_state = accounts.get(2)?;
        let pda_account = accounts.get(3)?;
        let pda_from_token_account = accounts.get(4)?;
        let buyer_from_token_account = accounts.get(5)?;
        let pda_to_token_account = accounts.get(6)?;
        let buyer_to_token_account = accounts.get(7)?;
        let swap_fee_account = accounts.get(8)?;
        let host_fee_account = accounts.get(9)?;
        let manager_fee_account = accounts.get(10)?;
        let token_list = accounts.get(11)?;
        let prism_data = accounts.get(12)?;
        let token_program = accounts.get(13)?;

        Some(SymmetrySwapInstructionAccounts {
            swap_program: *swap_program,
            buyer: *buyer,
            fund_state: *fund_state,
            pda_account: *pda_account,
            pda_from_token_account: *pda_from_token_account,
            buyer_from_token_account: *buyer_from_token_account,
            pda_to_token_account: *pda_to_token_account,
            buyer_to_token_account: *buyer_to_token_account,
            swap_fee_account: *swap_fee_account,
            host_fee_account: *host_fee_account,
            manager_fee_account: *manager_fee_account,
            token_list: *token_list,
            prism_data: *prism_data,
            token_program: *token_program,
        })
    }
}