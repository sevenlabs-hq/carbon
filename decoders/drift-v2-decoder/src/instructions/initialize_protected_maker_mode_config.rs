use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4367dc435820fc08")]
pub struct InitializeProtectedMakerModeConfig {
    pub max_users: u32,
}

pub struct InitializeProtectedMakerModeConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub protected_maker_mode_config: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeProtectedMakerModeConfig {
    type ArrangedAccounts = InitializeProtectedMakerModeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            admin,
            protected_maker_mode_config,
            state,
            rent,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeProtectedMakerModeConfigInstructionAccounts {
            admin: admin.pubkey,
            protected_maker_mode_config: protected_maker_mode_config.pubkey,
            state: state.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
