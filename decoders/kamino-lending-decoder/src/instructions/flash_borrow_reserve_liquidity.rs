use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x87e734a70734d4c1")]
pub struct FlashBorrowReserveLiquidity {
    pub liquidity_amount: u64,
}

pub struct FlashBorrowReserveLiquidityInstructionAccounts {
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_source_liquidity: solana_pubkey::Pubkey,
    pub user_destination_liquidity: solana_pubkey::Pubkey,
    pub reserve_liquidity_fee_receiver: solana_pubkey::Pubkey,
    pub referrer_token_state: solana_pubkey::Pubkey,
    pub referrer_account: solana_pubkey::Pubkey,
    pub sysvar_info: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FlashBorrowReserveLiquidity {
    type ArrangedAccounts = FlashBorrowReserveLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user_transfer_authority, lending_market_authority, lending_market, reserve, reserve_liquidity_mint, reserve_source_liquidity, user_destination_liquidity, reserve_liquidity_fee_receiver, referrer_token_state, referrer_account, sysvar_info, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
