use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaa734d0ba19df7a9")]
pub struct ToggleZetaGroupPerpsOnly {}

pub struct ToggleZetaGroupPerpsOnlyInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ToggleZetaGroupPerpsOnly {
    type ArrangedAccounts = ToggleZetaGroupPerpsOnlyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ToggleZetaGroupPerpsOnlyInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
