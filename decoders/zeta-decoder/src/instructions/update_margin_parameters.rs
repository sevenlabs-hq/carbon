use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4532aec57bc448ec")]
pub struct UpdateMarginParameters {
    pub args: UpdateMarginParametersArgs,
    pub asset: Asset,
}

pub struct UpdateMarginParametersInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMarginParameters {
    type ArrangedAccounts = UpdateMarginParametersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMarginParametersInstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            admin: admin.pubkey,
        })
    }
}
