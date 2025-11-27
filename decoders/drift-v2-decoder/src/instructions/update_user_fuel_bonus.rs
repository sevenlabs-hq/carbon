use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x58afc9bede648f39")]
pub struct UpdateUserFuelBonus {}

pub struct UpdateUserFuelBonusInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserFuelBonus {
    type ArrangedAccounts = UpdateUserFuelBonusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, user, user_stats, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserFuelBonusInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
