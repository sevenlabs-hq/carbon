

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x797f12cc49f5e141")]
pub struct BorrowObligationLiquidity{
    pub liquidity_amount: u64,
}

pub struct BorrowObligationLiquidityInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub borrow_reserve: solana_sdk::pubkey::Pubkey,
    pub borrow_reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_source_liquidity: solana_sdk::pubkey::Pubkey,
    pub borrow_reserve_liquidity_fee_receiver: solana_sdk::pubkey::Pubkey,
    pub user_destination_liquidity: solana_sdk::pubkey::Pubkey,
    pub referrer_token_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BorrowObligationLiquidity {
    type ArrangedAccounts = BorrowObligationLiquidityInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let lending_market_authority = accounts.get(3)?;
        let borrow_reserve = accounts.get(4)?;
        let borrow_reserve_liquidity_mint = accounts.get(5)?;
        let reserve_source_liquidity = accounts.get(6)?;
        let borrow_reserve_liquidity_fee_receiver = accounts.get(7)?;
        let user_destination_liquidity = accounts.get(8)?;
        let referrer_token_state = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let instruction_sysvar_account = accounts.get(11)?;

        Some(BorrowObligationLiquidityInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            borrow_reserve: borrow_reserve.pubkey,
            borrow_reserve_liquidity_mint: borrow_reserve_liquidity_mint.pubkey,
            reserve_source_liquidity: reserve_source_liquidity.pubkey,
            borrow_reserve_liquidity_fee_receiver: borrow_reserve_liquidity_fee_receiver.pubkey,
            user_destination_liquidity: user_destination_liquidity.pubkey,
            referrer_token_state: referrer_token_state.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}