use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe5c2d4ac080a8693")]
pub struct CreateOpenOrders {}

pub struct CreateOpenOrdersInstructionAccounts {
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOpenOrders {
    type ArrangedAccounts = CreateOpenOrdersInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let open_orders = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let dex_program = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let rent = accounts.get(4)?;
        let market = accounts.get(5)?;

        Some(CreateOpenOrdersInstructionAccounts {
            open_orders: open_orders.pubkey,
            payer: payer.pubkey,
            dex_program: dex_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            market: market.pubkey,
        })
    }
}
