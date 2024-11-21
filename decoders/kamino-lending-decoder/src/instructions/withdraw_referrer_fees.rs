

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xab7679c9e98c17e4")]
pub struct WithdrawReferrerFees{
}

pub struct WithdrawReferrerFeesInstructionAccounts {
    pub referrer: solana_sdk::pubkey::Pubkey,
    pub referrer_token_state: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_supply_liquidity: solana_sdk::pubkey::Pubkey,
    pub referrer_token_account: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawReferrerFees {
    type ArrangedAccounts = WithdrawReferrerFeesInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let referrer = accounts.get(0)?;
        let referrer_token_state = accounts.get(1)?;
        let reserve = accounts.get(2)?;
        let reserve_liquidity_mint = accounts.get(3)?;
        let reserve_supply_liquidity = accounts.get(4)?;
        let referrer_token_account = accounts.get(5)?;
        let lending_market = accounts.get(6)?;
        let lending_market_authority = accounts.get(7)?;
        let token_program = accounts.get(8)?;

        Some(WithdrawReferrerFeesInstructionAccounts {
            referrer: referrer.pubkey,
            referrer_token_state: referrer_token_state.pubkey,
            reserve: reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_supply_liquidity: reserve_supply_liquidity.pubkey,
            referrer_token_account: referrer_token_account.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}