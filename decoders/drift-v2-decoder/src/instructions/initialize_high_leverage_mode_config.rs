use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd5a75df6d0825af8")]
pub struct InitializeHighLeverageModeConfig {
    pub max_users: u32,
}

pub struct InitializeHighLeverageModeConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub high_leverage_mode_config: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeHighLeverageModeConfig {
    type ArrangedAccounts = InitializeHighLeverageModeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, high_leverage_mode_config, state, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeHighLeverageModeConfigInstructionAccounts {
            admin: admin.pubkey,
            high_leverage_mode_config: high_leverage_mode_config.pubkey,
            state: state.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
