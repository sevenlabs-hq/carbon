use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb1479abce2854a37")]
pub struct LiquidateObligationAndRedeemReserveCollateral {
    pub liquidity_amount: u64,
    pub min_acceptable_received_liquidity_amount: u64,
    pub max_allowed_ltv_override_percent: u64,
}

pub struct LiquidateObligationAndRedeemReserveCollateralInstructionAccounts {
    pub liquidator: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub repay_reserve: solana_pubkey::Pubkey,
    pub repay_reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub repay_reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub withdraw_reserve: solana_pubkey::Pubkey,
    pub withdraw_reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub withdraw_reserve_collateral_mint: solana_pubkey::Pubkey,
    pub withdraw_reserve_collateral_supply: solana_pubkey::Pubkey,
    pub withdraw_reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub withdraw_reserve_liquidity_fee_receiver: solana_pubkey::Pubkey,
    pub user_source_liquidity: solana_pubkey::Pubkey,
    pub user_destination_collateral: solana_pubkey::Pubkey,
    pub user_destination_liquidity: solana_pubkey::Pubkey,
    pub collateral_token_program: solana_pubkey::Pubkey,
    pub repay_liquidity_token_program: solana_pubkey::Pubkey,
    pub withdraw_liquidity_token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateObligationAndRedeemReserveCollateral {
    type ArrangedAccounts = LiquidateObligationAndRedeemReserveCollateralInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [liquidator, obligation, lending_market, lending_market_authority, repay_reserve, repay_reserve_liquidity_mint, repay_reserve_liquidity_supply, withdraw_reserve, withdraw_reserve_liquidity_mint, withdraw_reserve_collateral_mint, withdraw_reserve_collateral_supply, withdraw_reserve_liquidity_supply, withdraw_reserve_liquidity_fee_receiver, user_source_liquidity, user_destination_collateral, user_destination_liquidity, collateral_token_program, repay_liquidity_token_program, withdraw_liquidity_token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            LiquidateObligationAndRedeemReserveCollateralInstructionAccounts {
                liquidator: liquidator.pubkey,
                obligation: obligation.pubkey,
                lending_market: lending_market.pubkey,
                lending_market_authority: lending_market_authority.pubkey,
                repay_reserve: repay_reserve.pubkey,
                repay_reserve_liquidity_mint: repay_reserve_liquidity_mint.pubkey,
                repay_reserve_liquidity_supply: repay_reserve_liquidity_supply.pubkey,
                withdraw_reserve: withdraw_reserve.pubkey,
                withdraw_reserve_liquidity_mint: withdraw_reserve_liquidity_mint.pubkey,
                withdraw_reserve_collateral_mint: withdraw_reserve_collateral_mint.pubkey,
                withdraw_reserve_collateral_supply: withdraw_reserve_collateral_supply.pubkey,
                withdraw_reserve_liquidity_supply: withdraw_reserve_liquidity_supply.pubkey,
                withdraw_reserve_liquidity_fee_receiver: withdraw_reserve_liquidity_fee_receiver
                    .pubkey,
                user_source_liquidity: user_source_liquidity.pubkey,
                user_destination_collateral: user_destination_collateral.pubkey,
                user_destination_liquidity: user_destination_liquidity.pubkey,
                collateral_token_program: collateral_token_program.pubkey,
                repay_liquidity_token_program: repay_liquidity_token_program.pubkey,
                withdraw_liquidity_token_program: withdraw_liquidity_token_program.pubkey,
                instruction_sysvar_account: instruction_sysvar_account.pubkey,
            },
        )
    }
}
