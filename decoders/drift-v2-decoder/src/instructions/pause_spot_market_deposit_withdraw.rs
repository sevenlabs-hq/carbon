use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb7773baa8923f256")]
pub struct PauseSpotMarketDepositWithdraw {}

pub struct PauseSpotMarketDepositWithdrawInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub keeper: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PauseSpotMarketDepositWithdraw {
    type ArrangedAccounts = PauseSpotMarketDepositWithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            keeper,
            spot_market,
            spot_market_vault,
            _remaining @ ..,
        ] = accounts
        else {
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
