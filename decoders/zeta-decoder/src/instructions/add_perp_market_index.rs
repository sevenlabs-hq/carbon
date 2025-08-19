use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7a280e40a912e788")]
pub struct AddPerpMarketIndex {
    pub asset: Asset,
}

pub struct AddPerpMarketIndexInstructionAccounts {
    pub market_indexes: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddPerpMarketIndex {
    type ArrangedAccounts = AddPerpMarketIndexInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [market_indexes, pricing, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AddPerpMarketIndexInstructionAccounts {
            market_indexes: market_indexes.pubkey,
            pricing: pricing.pubkey,
        })
    }
}
