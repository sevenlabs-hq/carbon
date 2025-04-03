use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4b5d5ddc2296dac4")]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateral {
    pub collateral_amount: u64,
}

pub struct WithdrawObligationCollateralAndRedeemReserveCollateralInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub withdraw_reserve: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_source_collateral: solana_pubkey::Pubkey,
    pub reserve_collateral_mint: solana_pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub user_destination_liquidity: solana_pubkey::Pubkey,
    pub placeholder_user_destination_collateral: solana_pubkey::Pubkey,
    pub collateral_token_program: solana_pubkey::Pubkey,
    pub liquidity_token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts
    for WithdrawObligationCollateralAndRedeemReserveCollateral
{
    type ArrangedAccounts =
        WithdrawObligationCollateralAndRedeemReserveCollateralInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, obligation, lending_market, lending_market_authority, withdraw_reserve, reserve_liquidity_mint, reserve_source_collateral, reserve_collateral_mint, reserve_liquidity_supply, user_destination_liquidity, placeholder_user_destination_collateral, collateral_token_program, liquidity_token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            WithdrawObligationCollateralAndRedeemReserveCollateralInstructionAccounts {
                owner: owner.pubkey,
                obligation: obligation.pubkey,
                lending_market: lending_market.pubkey,
                lending_market_authority: lending_market_authority.pubkey,
                withdraw_reserve: withdraw_reserve.pubkey,
                reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
                reserve_source_collateral: reserve_source_collateral.pubkey,
                reserve_collateral_mint: reserve_collateral_mint.pubkey,
                reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
                user_destination_liquidity: user_destination_liquidity.pubkey,
                placeholder_user_destination_collateral: placeholder_user_destination_collateral
                    .pubkey,
                collateral_token_program: collateral_token_program.pubkey,
                liquidity_token_program: liquidity_token_program.pubkey,
                instruction_sysvar_account: instruction_sysvar_account.pubkey,
            },
        )
    }
}
