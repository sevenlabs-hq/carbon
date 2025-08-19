use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x504815bf1d792d6f")]
pub struct SetPerpetualsConfig {
    pub params: SetPerpetualsConfigParams,
}

pub struct SetPerpetualsConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPerpetualsConfig {
    type ArrangedAccounts = SetPerpetualsConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, perpetuals, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPerpetualsConfigInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
        })
    }
}
