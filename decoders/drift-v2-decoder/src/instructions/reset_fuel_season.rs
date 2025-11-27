use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc77ac0ff20633fc8")]
pub struct ResetFuelSeason {}

pub struct ResetFuelSeasonInstructionAccounts {
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResetFuelSeason {
    type ArrangedAccounts = ResetFuelSeasonInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user_stats, authority, state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ResetFuelSeasonInstructionAccounts {
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
