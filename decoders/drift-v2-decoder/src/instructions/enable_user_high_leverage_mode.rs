use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe718e670c9ad49b8")]
pub struct EnableUserHighLeverageMode {
    pub sub_account_id: u16,
}

pub struct EnableUserHighLeverageModeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub high_leverage_mode_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EnableUserHighLeverageMode {
    type ArrangedAccounts = EnableUserHighLeverageModeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, high_leverage_mode_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableUserHighLeverageModeInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
            high_leverage_mode_config: high_leverage_mode_config.pubkey,
        })
    }
}
