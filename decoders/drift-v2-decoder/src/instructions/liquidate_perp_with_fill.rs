use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f6f7c6956a9bb22")]
pub struct LiquidatePerpWithFill {
    pub market_index: u16,
}

pub struct LiquidatePerpWithFillInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub liquidator_stats: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidatePerpWithFill {
    type ArrangedAccounts = LiquidatePerpWithFillInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            authority,
            liquidator,
            liquidator_stats,
            user,
            user_stats,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(LiquidatePerpWithFillInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_stats: liquidator_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
