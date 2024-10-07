use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1ce22094bc8871ab")]
pub struct CreateProgramOpenOrders {
    pub id: u8,
}

pub struct CreateProgramOpenOrdersInstructionAccounts {
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateProgramOpenOrders {
    type ArrangedAccounts = CreateProgramOpenOrdersInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let open_orders = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let program_authority = accounts.get(2)?;
        let dex_program = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let rent = accounts.get(5)?;
        let market = accounts.get(6)?;

        Some(CreateProgramOpenOrdersInstructionAccounts {
            open_orders: *open_orders,
            payer: *payer,
            program_authority: *program_authority,
            dex_program: *dex_program,
            system_program: *system_program,
            rent: *rent,
            market: *market,
        })
    }
}
