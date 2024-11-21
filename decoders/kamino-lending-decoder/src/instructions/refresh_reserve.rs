

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x02da8aeb4fc91966")]
pub struct RefreshReserve{
}

pub struct RefreshReserveInstructionAccounts {
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub pyth_oracle: solana_sdk::pubkey::Pubkey,
    pub switchboard_price_oracle: solana_sdk::pubkey::Pubkey,
    pub switchboard_twap_oracle: solana_sdk::pubkey::Pubkey,
    pub scope_prices: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshReserve {
    type ArrangedAccounts = RefreshReserveInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let reserve = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let pyth_oracle = accounts.get(2)?;
        let switchboard_price_oracle = accounts.get(3)?;
        let switchboard_twap_oracle = accounts.get(4)?;
        let scope_prices = accounts.get(5)?;

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