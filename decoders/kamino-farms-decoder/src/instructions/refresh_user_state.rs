use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01870c3ef38c4d6c")]
pub struct RefreshUserState {}

pub struct RefreshUserStateInstructionAccounts {
    pub user_state: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshUserState {
    type ArrangedAccounts = RefreshUserStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user_state, farm_state, scope_prices, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RefreshUserStateInstructionAccounts {
            user_state: user_state.pubkey,
            farm_state: farm_state.pubkey,
            scope_prices: scope_prices.pubkey,
        })
    }
}
