use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xccb5afde287dbc47")]
pub struct CreateOpenOrdersAccount {
    pub name: String,
}

pub struct CreateOpenOrdersAccountInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub delegate_account: solana_pubkey::Pubkey,
    pub open_orders_indexer: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOpenOrdersAccount {
    type ArrangedAccounts = CreateOpenOrdersAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, owner, delegate_account, open_orders_indexer, open_orders_account, market, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
