use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x83700a3b203628a4")]
pub struct UpdateOracleGuardRails {
    pub oracle_guard_rails: OracleGuardRails,
}

pub struct UpdateOracleGuardRailsInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateOracleGuardRails {
    type ArrangedAccounts = UpdateOracleGuardRailsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOracleGuardRailsInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
