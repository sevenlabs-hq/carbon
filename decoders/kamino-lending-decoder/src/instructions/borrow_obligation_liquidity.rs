use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x797f12cc49f5e141")]
pub struct BorrowObligationLiquidity {
    pub liquidity_amount: u64,
}

pub struct BorrowObligationLiquidityInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub borrow_reserve: solana_pubkey::Pubkey,
    pub borrow_reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_source_liquidity: solana_pubkey::Pubkey,
    pub borrow_reserve_liquidity_fee_receiver: solana_pubkey::Pubkey,
    pub user_destination_liquidity: solana_pubkey::Pubkey,
    pub referrer_token_state: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BorrowObligationLiquidity {
    type ArrangedAccounts = BorrowObligationLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, obligation, lending_market, lending_market_authority, borrow_reserve, borrow_reserve_liquidity_mint, reserve_source_liquidity, borrow_reserve_liquidity_fee_receiver, user_destination_liquidity, referrer_token_state, token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
