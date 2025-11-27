use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d")]
pub struct WriteCollectionExternalPluginAdapterDataV1 {
    pub write_collection_external_plugin_adapter_data_v1_args:
        WriteCollectionExternalPluginAdapterDataV1Args,
}

pub struct WriteCollectionExternalPluginAdapterDataV1InstructionAccounts {
    pub collection: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub buffer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WriteCollectionExternalPluginAdapterDataV1 {
    type ArrangedAccounts = WriteCollectionExternalPluginAdapterDataV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            collection,
            payer,
            authority,
            buffer,
            system_program,
            log_wrapper,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(
            WriteCollectionExternalPluginAdapterDataV1InstructionAccounts {
                collection: collection.pubkey,
                payer: payer.pubkey,
                authority: authority.pubkey,
                buffer: buffer.pubkey,
                system_program: system_program.pubkey,
                log_wrapper: log_wrapper.pubkey,
            },
        )
    }
}
