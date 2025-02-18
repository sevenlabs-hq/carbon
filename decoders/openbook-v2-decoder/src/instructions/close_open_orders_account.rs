use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb04a73d236b35b67")]
pub struct CloseOpenOrdersAccount {}

pub struct CloseOpenOrdersAccountInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub open_orders_indexer: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub sol_destination: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOpenOrdersAccount {
    type ArrangedAccounts = CloseOpenOrdersAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let open_orders_indexer = accounts.get(1)?;
        let open_orders_account = accounts.get(2)?;
        let sol_destination = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CloseOpenOrdersAccountInstructionAccounts {
            owner: owner.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            sol_destination: sol_destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
