use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b5a61d111730728")]
pub struct ConfigValidatorSystem {
    pub extra_runs: u32,
}

pub struct ConfigValidatorSystemInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub manager_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigValidatorSystem {
    type ArrangedAccounts = ConfigValidatorSystemInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, manager_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ConfigValidatorSystemInstructionAccounts {
            state: state.pubkey,
            manager_authority: manager_authority.pubkey,
        })
    }
}
