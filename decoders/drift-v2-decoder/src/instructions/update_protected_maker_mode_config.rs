use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x56a6ebfd43cadf11")]
pub struct UpdateProtectedMakerModeConfig {
    pub max_users: u32,
    pub reduce_only: bool,
}

pub struct UpdateProtectedMakerModeConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub protected_maker_mode_config: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateProtectedMakerModeConfig {
    type ArrangedAccounts = UpdateProtectedMakerModeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, protected_maker_mode_config, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateProtectedMakerModeConfigInstructionAccounts {
            admin: admin.pubkey,
            protected_maker_mode_config: protected_maker_mode_config.pubkey,
            state: state.pubkey,
        })
    }
}
