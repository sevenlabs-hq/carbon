use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xea75b57db98edc1d")]
pub struct RedeemReserveCollateral {
    pub collateral_amount: u64,
}

pub struct RedeemReserveCollateralInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_collateral_mint: solana_pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub user_source_collateral: solana_pubkey::Pubkey,
    pub user_destination_liquidity: solana_pubkey::Pubkey,
    pub collateral_token_program: solana_pubkey::Pubkey,
    pub liquidity_token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RedeemReserveCollateral {
    type ArrangedAccounts = RedeemReserveCollateralInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, lending_market, reserve, lending_market_authority, reserve_liquidity_mint, reserve_collateral_mint, reserve_liquidity_supply, user_source_collateral, user_destination_liquidity, collateral_token_program, liquidity_token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RedeemReserveCollateralInstructionAccounts {
            owner: owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            user_source_collateral: user_source_collateral.pubkey,
            user_destination_liquidity: user_destination_liquidity.pubkey,
            collateral_token_program: collateral_token_program.pubkey,
            liquidity_token_program: liquidity_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
