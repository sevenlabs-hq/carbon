use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3013fed1c8d3313d")]
pub struct ResetNumFlexUnderlyings {}

pub struct ResetNumFlexUnderlyingsInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResetNumFlexUnderlyings {
    type ArrangedAccounts = ResetNumFlexUnderlyingsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ResetNumFlexUnderlyingsInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
