

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x22a2740e65895eef")]
pub struct InitLendingMarket{
    pub quote_currency: [u8; 32],
}

pub struct InitLendingMarketInstructionAccounts {
    pub lending_market_owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitLendingMarket {
    type ArrangedAccounts = InitLendingMarketInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let lending_market_owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let lending_market_authority = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let rent = accounts.get(4)?;

        Some(InitLendingMarketInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}