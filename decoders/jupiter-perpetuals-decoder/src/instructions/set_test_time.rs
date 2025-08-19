use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf2e7b1fb7e919f68")]
pub struct SetTestTime {
    pub params: SetTestTimeParams,
}

pub struct SetTestTimeInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTestTime {
    type ArrangedAccounts = SetTestTimeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, perpetuals, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTestTimeInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
        })
    }
}
