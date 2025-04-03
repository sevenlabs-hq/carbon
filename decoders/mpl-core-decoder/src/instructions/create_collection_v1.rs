use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01")]
pub struct CreateCollectionV1 {
    pub create_collection_v1_args: CreateCollectionV1Args,
}

pub struct CreateCollectionV1InstructionAccounts {
    pub collection: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCollectionV1 {
    type ArrangedAccounts = CreateCollectionV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection, update_authority, payer, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(CreateCollectionV1InstructionAccounts {
            collection: collection.pubkey,
            update_authority: update_authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
