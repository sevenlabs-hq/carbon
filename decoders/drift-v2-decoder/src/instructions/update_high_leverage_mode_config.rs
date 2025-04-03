use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x407ad45d8dd9ca37")]
pub struct UpdateHighLeverageModeConfig {
    pub max_users: u32,
    pub reduce_only: bool,
}

pub struct UpdateHighLeverageModeConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub high_leverage_mode_config: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateHighLeverageModeConfig {
    type ArrangedAccounts = UpdateHighLeverageModeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, high_leverage_mode_config, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateHighLeverageModeConfigInstructionAccounts {
            admin: admin.pubkey,
            high_leverage_mode_config: high_leverage_mode_config.pubkey,
            state: state.pubkey,
        })
    }
}
