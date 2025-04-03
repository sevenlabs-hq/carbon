use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5ef690af04a4e9fc")]
pub struct AddMarketIndexes {}

pub struct AddMarketIndexesInstructionAccounts {
    pub market_indexes: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddMarketIndexes {
    type ArrangedAccounts = AddMarketIndexesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [market_indexes, zeta_group, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AddMarketIndexesInstructionAccounts {
            market_indexes: market_indexes.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
