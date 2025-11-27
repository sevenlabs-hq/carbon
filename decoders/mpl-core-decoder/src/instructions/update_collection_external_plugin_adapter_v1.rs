use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct UpdateCollectionExternalPluginAdapterV1 {
    pub update_collection_external_plugin_adapter_v1_args:
        UpdateCollectionExternalPluginAdapterV1Args,
}

pub struct UpdateCollectionExternalPluginAdapterV1InstructionAccounts {
    pub collection: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCollectionExternalPluginAdapterV1 {
    type ArrangedAccounts = UpdateCollectionExternalPluginAdapterV1InstructionAccounts;

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

        Some(UpdateCollectionExternalPluginAdapterV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
