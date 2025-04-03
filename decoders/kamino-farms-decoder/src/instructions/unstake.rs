use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5a5f6b2acd7c32e1")]
pub struct Unstake {
    pub stake_shares_scaled: u128,
}

pub struct UnstakeInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub user_state: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Unstake {
    type ArrangedAccounts = UnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, user_state, farm_state, scope_prices, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UnstakeInstructionAccounts {
            owner: owner.pubkey,
            user_state: user_state.pubkey,
            farm_state: farm_state.pubkey,
            scope_prices: scope_prices.pubkey,
        })
    }
}
