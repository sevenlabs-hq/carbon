use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xccb5afde287dbc47")]
pub struct CreateOpenOrdersAccount {
    pub name: String,
}

pub struct CreateOpenOrdersAccountInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub delegate_account: solana_sdk::pubkey::Pubkey,
    pub open_orders_indexer: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOpenOrdersAccount {
    type ArrangedAccounts = CreateOpenOrdersAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let delegate_account = accounts.get(2)?;
        let open_orders_indexer = accounts.get(3)?;
        let open_orders_account = accounts.get(4)?;
        let market = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(CreateOpenOrdersAccountInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            delegate_account: delegate_account.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            market: market.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
