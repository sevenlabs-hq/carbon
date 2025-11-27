use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x404099ffd947f985")]
pub struct CreateOpenOrdersIndexer {}

pub struct CreateOpenOrdersIndexerInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub open_orders_indexer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOpenOrdersIndexer {
    type ArrangedAccounts = CreateOpenOrdersIndexerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, owner, open_orders_indexer, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateOpenOrdersIndexerInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
