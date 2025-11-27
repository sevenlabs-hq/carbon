use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5c28972a7afe8bf6")]
pub struct DepositIntoSpotMarketRevenuePool {
    pub amount: u64,
}

pub struct DepositIntoSpotMarketRevenuePoolInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositIntoSpotMarketRevenuePool {
    type ArrangedAccounts = DepositIntoSpotMarketRevenuePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, authority, spot_market_vault, user_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositIntoSpotMarketRevenuePoolInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            authority: authority.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            user_token_account: user_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
