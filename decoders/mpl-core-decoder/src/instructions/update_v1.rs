use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f")]
pub struct UpdateV1 {
    pub update_v1_args: UpdateV1Args,
}

pub struct UpdateV1InstructionAccounts {
    pub asset: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateV1 {
    type ArrangedAccounts = UpdateV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [asset, collection, payer, authority, system_program, log_wrapper, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdateV1InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
