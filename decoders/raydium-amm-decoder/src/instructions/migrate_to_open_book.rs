
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcf62f35972aecd14")]
pub struct MigrateToOpenBook{
}

pub struct MigrateToOpenBookInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_token_coin: solana_sdk::pubkey::Pubkey,
    pub amm_token_pc: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub new_amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub new_serum_program: solana_sdk::pubkey::Pubkey,
    pub new_serum_market: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MigrateToOpenBook {
    type ArrangedAccounts = MigrateToOpenBookInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let system_program = accounts.get(1)?;
        let rent = accounts.get(2)?;
        let amm = accounts.get(3)?;
        let amm_authority = accounts.get(4)?;
        let amm_open_orders = accounts.get(5)?;
        let amm_token_coin = accounts.get(6)?;
        let amm_token_pc = accounts.get(7)?;
        let amm_target_orders = accounts.get(8)?;
        let serum_program = accounts.get(9)?;
        let serum_market = accounts.get(10)?;
        let serum_bids = accounts.get(11)?;
        let serum_asks = accounts.get(12)?;
        let serum_event_queue = accounts.get(13)?;
        let serum_coin_vault = accounts.get(14)?;
        let serum_pc_vault = accounts.get(15)?;
        let serum_vault_signer = accounts.get(16)?;
        let new_amm_open_orders = accounts.get(17)?;
        let new_serum_program = accounts.get(18)?;
        let new_serum_market = accounts.get(19)?;
        let admin = accounts.get(20)?;

        Some(MigrateToOpenBookInstructionAccounts {
            token_program: *token_program,
            system_program: *system_program,
            rent: *rent,
            amm: *amm,
            amm_authority: *amm_authority,
            amm_open_orders: *amm_open_orders,
            amm_token_coin: *amm_token_coin,
            amm_token_pc: *amm_token_pc,
            amm_target_orders: *amm_target_orders,
            serum_program: *serum_program,
            serum_market: *serum_market,
            serum_bids: *serum_bids,
            serum_asks: *serum_asks,
            serum_event_queue: *serum_event_queue,
            serum_coin_vault: *serum_coin_vault,
            serum_pc_vault: *serum_pc_vault,
            serum_vault_signer: *serum_vault_signer,
            new_amm_open_orders: *new_amm_open_orders,
            new_serum_program: *new_serum_program,
            new_serum_market: *new_serum_market,
            admin: *admin,
        })
    }
}