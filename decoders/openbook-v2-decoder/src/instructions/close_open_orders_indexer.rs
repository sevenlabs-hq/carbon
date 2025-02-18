use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x67f9e5e7f7fdc588")]
pub struct CloseOpenOrdersIndexer {}

pub struct CloseOpenOrdersIndexerInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub open_orders_indexer: solana_sdk::pubkey::Pubkey,
    pub sol_destination: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOpenOrdersIndexer {
    type ArrangedAccounts = CloseOpenOrdersIndexerInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let open_orders_indexer = accounts.get(1)?;
        let sol_destination = accounts.get(2)?;
        let token_program = accounts.get(3)?;

        Some(CloseOpenOrdersIndexerInstructionAccounts {
            owner: owner.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            sol_destination: sol_destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
