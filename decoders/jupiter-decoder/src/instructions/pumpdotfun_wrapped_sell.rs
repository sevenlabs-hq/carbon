
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xff136363284153ff")]
pub struct PumpdotfunWrappedSell{
}

pub struct PumpdotfunWrappedSellInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub global: solana_sdk::pubkey::Pubkey,
    pub fee_recipient: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_user: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub user_wsol_token_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for PumpdotfunWrappedSell {
    type ArrangedAccounts = PumpdotfunWrappedSellInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let global = accounts.get(1)?;
        let fee_recipient = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let bonding_curve = accounts.get(4)?;
        let associated_bonding_curve = accounts.get(5)?;
        let associated_user = accounts.get(6)?;
        let user = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;
        let user_wsol_token_account = accounts.get(13)?;

        Some(PumpdotfunWrappedSellInstructionAccounts {
            swap_program: *swap_program,
            global: *global,
            fee_recipient: *fee_recipient,
            mint: *mint,
            bonding_curve: *bonding_curve,
            associated_bonding_curve: *associated_bonding_curve,
            associated_user: *associated_user,
            user: *user,
            system_program: *system_program,
            associated_token_program: *associated_token_program,
            token_program: *token_program,
            event_authority: *event_authority,
            program: *program,
            user_wsol_token_account: *user_wsol_token_account,
        })
    }
}