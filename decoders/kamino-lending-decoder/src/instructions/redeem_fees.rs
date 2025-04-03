use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd727b429ad2ef8dc")]
pub struct RedeemFees {}

pub struct RedeemFeesInstructionAccounts {
    pub reserve: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_liquidity_fee_receiver: solana_pubkey::Pubkey,
    pub reserve_supply_liquidity: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RedeemFees {
    type ArrangedAccounts = RedeemFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reserve, reserve_liquidity_mint, reserve_liquidity_fee_receiver, reserve_supply_liquidity, lending_market, lending_market_authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
