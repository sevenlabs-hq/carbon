

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd727b429ad2ef8dc")]
pub struct RedeemFees{
}

pub struct RedeemFeesInstructionAccounts {
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_fee_receiver: solana_sdk::pubkey::Pubkey,
    pub reserve_supply_liquidity: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RedeemFees {
    type ArrangedAccounts = RedeemFeesInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let reserve = accounts.get(0)?;
        let reserve_liquidity_mint = accounts.get(1)?;
        let reserve_liquidity_fee_receiver = accounts.get(2)?;
        let reserve_supply_liquidity = accounts.get(3)?;
        let lending_market = accounts.get(4)?;
        let lending_market_authority = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(RedeemFeesInstructionAccounts {
            reserve: reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_liquidity_fee_receiver: reserve_liquidity_fee_receiver.pubkey,
            reserve_supply_liquidity: reserve_supply_liquidity.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}