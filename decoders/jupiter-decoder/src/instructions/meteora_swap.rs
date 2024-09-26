
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7f7de20c5118cc23")]
pub struct MeteoraSwap{
}

pub struct MeteoraSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub user_source_token: solana_sdk::pubkey::Pubkey,
    pub user_destination_token: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_token_vault: solana_sdk::pubkey::Pubkey,
    pub b_token_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub admin_token_fee: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MeteoraSwap {
    type ArrangedAccounts = MeteoraSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let user_source_token = accounts.get(2)?;
        let user_destination_token = accounts.get(3)?;
        let a_vault = accounts.get(4)?;
        let b_vault = accounts.get(5)?;
        let a_token_vault = accounts.get(6)?;
        let b_token_vault = accounts.get(7)?;
        let a_vault_lp_mint = accounts.get(8)?;
        let b_vault_lp_mint = accounts.get(9)?;
        let a_vault_lp = accounts.get(10)?;
        let b_vault_lp = accounts.get(11)?;
        let admin_token_fee = accounts.get(12)?;
        let user = accounts.get(13)?;
        let vault_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;

        Some(MeteoraSwapInstructionAccounts {
            swap_program: *swap_program,
            pool: *pool,
            user_source_token: *user_source_token,
            user_destination_token: *user_destination_token,
            a_vault: *a_vault,
            b_vault: *b_vault,
            a_token_vault: *a_token_vault,
            b_token_vault: *b_token_vault,
            a_vault_lp_mint: *a_vault_lp_mint,
            b_vault_lp_mint: *b_vault_lp_mint,
            a_vault_lp: *a_vault_lp,
            b_vault_lp: *b_vault_lp,
            admin_token_fee: *admin_token_fee,
            user: *user,
            vault_program: *vault_program,
            token_program: *token_program,
        })
    }
}