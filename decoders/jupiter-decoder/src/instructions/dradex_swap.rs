
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8ad1b5d5b7dfcabb")]
pub struct DradexSwap{
}

pub struct DradexSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub pair: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub event_queue: solana_sdk::pubkey::Pubkey,
    pub dex_user: solana_sdk::pubkey::Pubkey,
    pub market_user: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub t0_vault: solana_sdk::pubkey::Pubkey,
    pub t1_vault: solana_sdk::pubkey::Pubkey,
    pub t0_user: solana_sdk::pubkey::Pubkey,
    pub t1_user: solana_sdk::pubkey::Pubkey,
    pub master: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub logger: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DradexSwap {
    type ArrangedAccounts = DradexSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let pair = accounts.get(1)?;
        let market = accounts.get(2)?;
        let event_queue = accounts.get(3)?;
        let dex_user = accounts.get(4)?;
        let market_user = accounts.get(5)?;
        let bids = accounts.get(6)?;
        let asks = accounts.get(7)?;
        let t0_vault = accounts.get(8)?;
        let t1_vault = accounts.get(9)?;
        let t0_user = accounts.get(10)?;
        let t1_user = accounts.get(11)?;
        let master = accounts.get(12)?;
        let signer = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let logger = accounts.get(16)?;

        Some(DradexSwapInstructionAccounts {
            swap_program: *swap_program,
            pair: *pair,
            market: *market,
            event_queue: *event_queue,
            dex_user: *dex_user,
            market_user: *market_user,
            bids: *bids,
            asks: *asks,
            t0_vault: *t0_vault,
            t1_vault: *t1_vault,
            t0_user: *t0_user,
            t1_user: *t1_user,
            master: *master,
            signer: *signer,
            system_program: *system_program,
            token_program: *token_program,
            logger: *logger,
        })
    }
}