use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14")]
pub struct CreateV2 {
    pub create_v2_args: CreateV2Args,
}

pub struct CreateV2InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateV2 {
    type ArrangedAccounts = CreateV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [asset, collection, authority, payer, owner, update_authority, system_program, log_wrapper, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateV2InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
