
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3eecf81cdee8b649")]
pub struct MarinadeDeposit{
}

pub struct MarinadeDepositInstructionAccounts {
    pub marinade_finance_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub transfer_from: solana_sdk::pubkey::Pubkey,
    pub mint_to: solana_sdk::pubkey::Pubkey,
    pub msol_mint_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub user_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MarinadeDeposit {
    type ArrangedAccounts = MarinadeDepositInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let marinade_finance_program = accounts.get(0)?;
        let state = accounts.get(1)?;
        let msol_mint = accounts.get(2)?;
        let liq_pool_sol_leg_pda = accounts.get(3)?;
        let liq_pool_msol_leg = accounts.get(4)?;
        let liq_pool_msol_leg_authority = accounts.get(5)?;
        let reserve_pda = accounts.get(6)?;
        let transfer_from = accounts.get(7)?;
        let mint_to = accounts.get(8)?;
        let msol_mint_authority = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let user_wsol_token_account = accounts.get(12)?;
        let temp_wsol_token_account = accounts.get(13)?;
        let user_transfer_authority = accounts.get(14)?;
        let payer = accounts.get(15)?;
        let wsol_mint = accounts.get(16)?;
        let rent = accounts.get(17)?;

        Some(MarinadeDepositInstructionAccounts {
            marinade_finance_program: *marinade_finance_program,
            state: *state,
            msol_mint: *msol_mint,
            liq_pool_sol_leg_pda: *liq_pool_sol_leg_pda,
            liq_pool_msol_leg: *liq_pool_msol_leg,
            liq_pool_msol_leg_authority: *liq_pool_msol_leg_authority,
            reserve_pda: *reserve_pda,
            transfer_from: *transfer_from,
            mint_to: *mint_to,
            msol_mint_authority: *msol_mint_authority,
            system_program: *system_program,
            token_program: *token_program,
            user_wsol_token_account: *user_wsol_token_account,
            temp_wsol_token_account: *temp_wsol_token_account,
            user_transfer_authority: *user_transfer_authority,
            payer: *payer,
            wsol_mint: *wsol_mint,
            rent: *rent,
        })
    }
}