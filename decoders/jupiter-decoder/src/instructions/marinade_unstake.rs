
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x29780f0071db2a01")]
pub struct MarinadeUnstake{
}

pub struct MarinadeUnstakeInstructionAccounts {
    pub marinade_finance_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub treasury_msol_account: solana_sdk::pubkey::Pubkey,
    pub get_msol_from: solana_sdk::pubkey::Pubkey,
    pub get_msol_from_authority: solana_sdk::pubkey::Pubkey,
    pub transfer_sol_to: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub user_wsol_token_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MarinadeUnstake {
    type ArrangedAccounts = MarinadeUnstakeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let marinade_finance_program = accounts.get(0)?;
        let state = accounts.get(1)?;
        let msol_mint = accounts.get(2)?;
        let liq_pool_sol_leg_pda = accounts.get(3)?;
        let liq_pool_msol_leg = accounts.get(4)?;
        let treasury_msol_account = accounts.get(5)?;
        let get_msol_from = accounts.get(6)?;
        let get_msol_from_authority = accounts.get(7)?;
        let transfer_sol_to = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let user_wsol_token_account = accounts.get(11)?;

        Some(MarinadeUnstakeInstructionAccounts {
            marinade_finance_program: *marinade_finance_program,
            state: *state,
            msol_mint: *msol_mint,
            liq_pool_sol_leg_pda: *liq_pool_sol_leg_pda,
            liq_pool_msol_leg: *liq_pool_msol_leg,
            treasury_msol_account: *treasury_msol_account,
            get_msol_from: *get_msol_from,
            get_msol_from_authority: *get_msol_from_authority,
            transfer_sol_to: *transfer_sol_to,
            system_program: *system_program,
            token_program: *token_program,
            user_wsol_token_account: *user_wsol_token_account,
        })
    }
}