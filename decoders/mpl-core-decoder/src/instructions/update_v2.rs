use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1e")]
pub struct UpdateV2 {
    pub update_v2_args: UpdateV2Args,
}

pub struct UpdateV2InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub new_collection: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateV2 {
    type ArrangedAccounts = UpdateV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [asset, collection, payer, authority, new_collection, system_program, log_wrapper] =
            accounts
        else {
            return None;
        };

        Some(UpdateV2InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            new_collection: new_collection.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
