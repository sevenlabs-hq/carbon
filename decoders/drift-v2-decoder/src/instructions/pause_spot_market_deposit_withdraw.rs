use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb7773baa8923f256")]
pub struct PauseSpotMarketDepositWithdraw {}

pub struct PauseSpotMarketDepositWithdrawInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
    pub spot_market_vault: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PauseSpotMarketDepositWithdraw {
    type ArrangedAccounts = PauseSpotMarketDepositWithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, keeper, spot_market, spot_market_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PauseSpotMarketDepositWithdrawInstructionAccounts {
            state: state.pubkey,
            keeper: keeper.pubkey,
            spot_market: spot_market.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
        })
    }
}
