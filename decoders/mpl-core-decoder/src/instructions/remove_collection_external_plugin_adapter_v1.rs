use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x19")]
pub struct RemoveCollectionExternalPluginAdapterV1 {
    pub remove_collection_external_plugin_adapter_v1_args:
        RemoveCollectionExternalPluginAdapterV1Args,
}

pub struct RemoveCollectionExternalPluginAdapterV1InstructionAccounts {
    pub collection: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCollectionExternalPluginAdapterV1 {
    type ArrangedAccounts = RemoveCollectionExternalPluginAdapterV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection, payer, authority, system_program, log_wrapper, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(RemoveCollectionExternalPluginAdapterV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
