use crate::accounts::fees::Fees;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

use super::super::types::*;
use carbon_core::borsh;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1beab2349302bb8d")]
pub struct SetParams {
    pub param: u8,
    pub value: Option<u64>,
    pub new_pubkey: Option<solana_sdk::pubkey::Pubkey>,
    pub fees: Option<Fees>,
    pub last_order_distance: Option<LastOrderDistance>,
    pub need_take_amounts: Option<NeedTake>,
}

pub struct SetParamsInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub amm_coin_vault: solana_sdk::pubkey::Pubkey,
    pub amm_pc_vault: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub amm_admin_account: solana_sdk::pubkey::Pubkey,
    pub new_amm_open_orders_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetParams {
    type ArrangedAccounts = SetParamsInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let amm = accounts.get(1)?;
        let amm_authority = accounts.get(2)?;
        let amm_open_orders = accounts.get(3)?;
        let amm_target_orders = accounts.get(4)?;
        let amm_coin_vault = accounts.get(5)?;
        let amm_pc_vault = accounts.get(6)?;
        let serum_program = accounts.get(7)?;
        let serum_market = accounts.get(8)?;
        let serum_coin_vault = accounts.get(9)?;
        let serum_pc_vault = accounts.get(10)?;
        let serum_vault_signer = accounts.get(11)?;
        let serum_event_queue = accounts.get(12)?;
        let serum_bids = accounts.get(13)?;
        let serum_asks = accounts.get(14)?;
        let amm_admin_account = accounts.get(15)?;
        let new_amm_open_orders_account = accounts.get(16)?;

        Some(SetParamsInstructionAccounts {
            token_program: *token_program,
            amm: *amm,
            amm_authority: *amm_authority,
            amm_open_orders: *amm_open_orders,
            amm_target_orders: *amm_target_orders,
            amm_coin_vault: *amm_coin_vault,
            amm_pc_vault: *amm_pc_vault,
            serum_program: *serum_program,
            serum_market: *serum_market,
            serum_coin_vault: *serum_coin_vault,
            serum_pc_vault: *serum_pc_vault,
            serum_vault_signer: *serum_vault_signer,
            serum_event_queue: *serum_event_queue,
            serum_bids: *serum_bids,
            serum_asks: *serum_asks,
            amm_admin_account: *amm_admin_account,
            new_amm_open_orders_account: *new_amm_open_orders_account,
        })
    }
}
