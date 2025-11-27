use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17")]
pub struct AddCollectionExternalPluginAdapterV1 {
    pub add_collection_external_plugin_adapter_v1_args: AddCollectionExternalPluginAdapterV1Args,
}

pub struct AddCollectionExternalPluginAdapterV1InstructionAccounts {
    pub collection: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddCollectionExternalPluginAdapterV1 {
    type ArrangedAccounts = AddCollectionExternalPluginAdapterV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            collection,
            payer,
            authority,
            system_program,
            log_wrapper,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(AddCollectionExternalPluginAdapterV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
