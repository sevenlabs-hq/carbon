use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xab7679c9e98c17e4")]
pub struct WithdrawReferrerFees {}

pub struct WithdrawReferrerFeesInstructionAccounts {
    pub referrer: solana_pubkey::Pubkey,
    pub referrer_token_state: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_supply_liquidity: solana_pubkey::Pubkey,
    pub referrer_token_account: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawReferrerFees {
    type ArrangedAccounts = WithdrawReferrerFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [referrer, referrer_token_state, reserve, reserve_liquidity_mint, reserve_supply_liquidity, referrer_token_account, lending_market, lending_market_authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
