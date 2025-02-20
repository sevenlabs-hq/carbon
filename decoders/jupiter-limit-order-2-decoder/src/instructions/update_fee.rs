use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe8fdc3f794d449de")]
pub struct UpdateFee {
    pub params: UpdateFeeParams,
}

pub struct UpdateFeeInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFee {
    type ArrangedAccounts = UpdateFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, fee_authority, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFeeInstructionAccounts {
            admin: admin.pubkey,
            fee_authority: fee_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
