
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4addb3d34913f3c4")]
pub struct CreateOpenOrders{
}

pub struct CreateOpenOrdersInstructionAccounts {
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateOpenOrders {
    type ArrangedAccounts = CreateOpenOrdersInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let open_orders = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let dex_program = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let rent = accounts.get(4)?;
        let market = accounts.get(5)?;

        Some(CreateOpenOrdersInstructionAccounts {
            open_orders: *open_orders,
            payer: *payer,
            dex_program: *dex_program,
            system_program: *system_program,
            rent: *rent,
            market: *market,
        })
    }
}