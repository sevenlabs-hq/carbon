use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct TransferV1 {
    pub transfer_v1_args: TransferV1Args,
}

pub struct TransferV1InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub new_owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferV1 {
    type ArrangedAccounts = TransferV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [asset, collection, payer, authority, new_owner, system_program, log_wrapper, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferV1InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            new_owner: new_owner.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
