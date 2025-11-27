use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct UpdatePluginV1 {
    pub update_plugin_v1_args: UpdatePluginV1Args,
}

pub struct UpdatePluginV1InstructionAccounts {
    pub asset: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePluginV1 {
    type ArrangedAccounts = UpdatePluginV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            asset,
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

        Some(UpdatePluginV1InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
