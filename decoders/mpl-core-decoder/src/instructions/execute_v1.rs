use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f")]
pub struct ExecuteV1 {
    pub execute_v1_args: ExecuteV1Args,
}

pub struct ExecuteV1InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub asset_signer: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub program_id: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExecuteV1 {
    type ArrangedAccounts = ExecuteV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [asset, collection, asset_signer, payer, authority, system_program, program_id, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ExecuteV1InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            asset_signer: asset_signer.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            program_id: program_id.pubkey,
        })
    }
}
