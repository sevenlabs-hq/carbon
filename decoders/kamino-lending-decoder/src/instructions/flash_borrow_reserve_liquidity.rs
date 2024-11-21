

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x87e734a70734d4c1")]
pub struct FlashBorrowReserveLiquidity{
    pub liquidity_amount: u64,
}

pub struct FlashBorrowReserveLiquidityInstructionAccounts {
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_source_liquidity: solana_sdk::pubkey::Pubkey,
    pub user_destination_liquidity: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_fee_receiver: solana_sdk::pubkey::Pubkey,
    pub referrer_token_state: solana_sdk::pubkey::Pubkey,
    pub referrer_account: solana_sdk::pubkey::Pubkey,
    pub sysvar_info: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FlashBorrowReserveLiquidity {
    type ArrangedAccounts = FlashBorrowReserveLiquidityInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let user_transfer_authority = accounts.get(0)?;
        let lending_market_authority = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let reserve = accounts.get(3)?;
        let reserve_liquidity_mint = accounts.get(4)?;
        let reserve_source_liquidity = accounts.get(5)?;
        let user_destination_liquidity = accounts.get(6)?;
        let reserve_liquidity_fee_receiver = accounts.get(7)?;
        let referrer_token_state = accounts.get(8)?;
        let referrer_account = accounts.get(9)?;
        let sysvar_info = accounts.get(10)?;
        let token_program = accounts.get(11)?;

        Some(FlashBorrowReserveLiquidityInstructionAccounts {
            user_transfer_authority: user_transfer_authority.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_source_liquidity: reserve_source_liquidity.pubkey,
            user_destination_liquidity: user_destination_liquidity.pubkey,
            reserve_liquidity_fee_receiver: reserve_liquidity_fee_receiver.pubkey,
            referrer_token_state: referrer_token_state.pubkey,
            referrer_account: referrer_account.pubkey,
            sysvar_info: sysvar_info.pubkey,
            token_program: token_program.pubkey,
        })
    }
}