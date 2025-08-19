use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4b08ff297b3b87ee")]
pub struct UpdateInterestRate {
    pub args: UpdateInterestRateArgs,
}

pub struct UpdateInterestRateInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateInterestRate {
    type ArrangedAccounts = UpdateInterestRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, greeks, zeta_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateInterestRateInstructionAccounts {
            state: state.pubkey,
            greeks: greeks.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
