use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdb8939165eba6072")]
pub struct RewardUserOnce {
    pub reward_index: u64,
    pub amount: u64,
}

pub struct RewardUserOnceInstructionAccounts {
    pub farm_admin: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub user_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RewardUserOnce {
    type ArrangedAccounts = RewardUserOnceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [farm_admin, farm_state, user_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RewardUserOnceInstructionAccounts {
            farm_admin: farm_admin.pubkey,
            farm_state: farm_state.pubkey,
            user_state: user_state.pubkey,
        })
    }
}
