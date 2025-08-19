use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbe6974dde5c6d053")]
pub struct UpdateVolatility {
    pub args: UpdateVolatilityArgs,
}

pub struct UpdateVolatilityInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateVolatility {
    type ArrangedAccounts = UpdateVolatilityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, greeks, zeta_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateVolatilityInstructionAccounts {
            state: state.pubkey,
            greeks: greeks.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
