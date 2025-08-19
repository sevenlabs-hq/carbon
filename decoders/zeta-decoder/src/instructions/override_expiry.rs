use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x81c575726c77cf88")]
pub struct OverrideExpiry {
    pub args: OverrideExpiryArgs,
}

pub struct OverrideExpiryInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OverrideExpiry {
    type ArrangedAccounts = OverrideExpiryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, zeta_group, _remaining @ ..] = accounts else {
            return None;
        };

        Some(OverrideExpiryInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
