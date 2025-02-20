use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0d")]
pub struct BurnCollectionV1 {
    pub burn_collection_v1_args: BurnCollectionV1Args,
}

pub struct BurnCollectionV1InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnCollectionV1 {
    type ArrangedAccounts = BurnCollectionV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection, payer, authority, log_wrapper, _remaining @ ..] = accounts else {
            return None;
        };

        Some(BurnCollectionV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
