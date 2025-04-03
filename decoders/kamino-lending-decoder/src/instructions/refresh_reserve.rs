use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02da8aeb4fc91966")]
pub struct RefreshReserve {}

pub struct RefreshReserveInstructionAccounts {
    pub reserve: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub pyth_oracle: solana_pubkey::Pubkey,
    pub switchboard_price_oracle: solana_pubkey::Pubkey,
    pub switchboard_twap_oracle: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshReserve {
    type ArrangedAccounts = RefreshReserveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reserve, lending_market, pyth_oracle, switchboard_price_oracle, switchboard_twap_oracle, scope_prices, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RefreshReserveInstructionAccounts {
            reserve: reserve.pubkey,
            lending_market: lending_market.pubkey,
            pyth_oracle: pyth_oracle.pubkey,
            switchboard_price_oracle: switchboard_price_oracle.pubkey,
            switchboard_twap_oracle: switchboard_twap_oracle.pubkey,
            scope_prices: scope_prices.pubkey,
        })
    }
}
