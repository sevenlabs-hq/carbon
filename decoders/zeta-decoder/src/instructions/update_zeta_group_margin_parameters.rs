use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3cd07993f26a0bfe")]
pub struct UpdateZetaGroupMarginParameters {
    pub args: UpdateMarginParametersArgs,
}

pub struct UpdateZetaGroupMarginParametersInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateZetaGroupMarginParameters {
    type ArrangedAccounts = UpdateZetaGroupMarginParametersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateZetaGroupMarginParametersInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
