use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb04a73d236b35b67")]
pub struct CloseOpenOrdersAccount {}

pub struct CloseOpenOrdersAccountInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub open_orders_indexer: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub sol_destination: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOpenOrdersAccount {
    type ArrangedAccounts = CloseOpenOrdersAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            owner,
            open_orders_indexer,
            open_orders_account,
            sol_destination,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CloseOpenOrdersAccountInstructionAccounts {
            owner: owner.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            sol_destination: sol_destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
