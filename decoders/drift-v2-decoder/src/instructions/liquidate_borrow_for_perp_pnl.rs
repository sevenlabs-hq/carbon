use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa911205acf94d11b")]
pub struct LiquidateBorrowForPerpPnl {
    pub perp_market_index: u16,
    pub spot_market_index: u16,
    pub liquidator_max_liability_transfer: u128,
    pub limit_price: Option<u64>,
}

pub struct LiquidateBorrowForPerpPnlInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub liquidator_stats: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateBorrowForPerpPnl {
    type ArrangedAccounts = LiquidateBorrowForPerpPnlInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, liquidator, liquidator_stats, user, user_stats, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidateBorrowForPerpPnlInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_stats: liquidator_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
