
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa9dc29fa23be85c6")]
pub struct CremaSwap{
}

pub struct CremaSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub clmm_config: solana_sdk::pubkey::Pubkey,
    pub clmmpool: solana_sdk::pubkey::Pubkey,
    pub token_a: solana_sdk::pubkey::Pubkey,
    pub token_b: solana_sdk::pubkey::Pubkey,
    pub account_a: solana_sdk::pubkey::Pubkey,
    pub account_b: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub tick_array_map: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub partner: solana_sdk::pubkey::Pubkey,
    pub partner_ata_a: solana_sdk::pubkey::Pubkey,
    pub partner_ata_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CremaSwap {
    type ArrangedAccounts = CremaSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let clmm_config = accounts.get(1)?;
        let clmmpool = accounts.get(2)?;
        let token_a = accounts.get(3)?;
        let token_b = accounts.get(4)?;
        let account_a = accounts.get(5)?;
        let account_b = accounts.get(6)?;
        let token_a_vault = accounts.get(7)?;
        let token_b_vault = accounts.get(8)?;
        let tick_array_map = accounts.get(9)?;
        let owner = accounts.get(10)?;
        let partner = accounts.get(11)?;
        let partner_ata_a = accounts.get(12)?;
        let partner_ata_b = accounts.get(13)?;
        let token_program = accounts.get(14)?;

        Some(CremaSwapInstructionAccounts {
            swap_program: *swap_program,
            clmm_config: *clmm_config,
            clmmpool: *clmmpool,
            token_a: *token_a,
            token_b: *token_b,
            account_a: *account_a,
            account_b: *account_b,
            token_a_vault: *token_a_vault,
            token_b_vault: *token_b_vault,
            tick_array_map: *tick_array_map,
            owner: *owner,
            partner: *partner,
            partner_ata_a: *partner_ata_a,
            partner_ata_b: *partner_ata_b,
            token_program: *token_program,
        })
    }
}