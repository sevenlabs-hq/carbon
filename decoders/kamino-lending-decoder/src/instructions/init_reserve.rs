

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8af547e19904032b")]
pub struct InitReserve{
}

pub struct InitReserveInstructionAccounts {
    pub lending_market_owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub fee_receiver: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_supply: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub liquidity_token_program: solana_sdk::pubkey::Pubkey,
    pub collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitReserve {
    type ArrangedAccounts = InitReserveInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let lending_market_owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let lending_market_authority = accounts.get(2)?;
        let reserve = accounts.get(3)?;
        let reserve_liquidity_mint = accounts.get(4)?;
        let reserve_liquidity_supply = accounts.get(5)?;
        let fee_receiver = accounts.get(6)?;
        let reserve_collateral_mint = accounts.get(7)?;
        let reserve_collateral_supply = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let liquidity_token_program = accounts.get(10)?;
        let collateral_token_program = accounts.get(11)?;
        let system_program = accounts.get(12)?;

        Some(InitReserveInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            fee_receiver: fee_receiver.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            reserve_collateral_supply: reserve_collateral_supply.pubkey,
            rent: rent.pubkey,
            liquidity_token_program: liquidity_token_program.pubkey,
            collateral_token_program: collateral_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}