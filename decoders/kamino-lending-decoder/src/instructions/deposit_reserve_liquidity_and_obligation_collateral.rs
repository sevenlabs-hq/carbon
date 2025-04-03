use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x81c70402de271a2e")]
pub struct DepositReserveLiquidityAndObligationCollateral {
    pub liquidity_amount: u64,
}

pub struct DepositReserveLiquidityAndObligationCollateralInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub reserve_collateral_mint: solana_pubkey::Pubkey,
    pub reserve_destination_deposit_collateral: solana_pubkey::Pubkey,
    pub user_source_liquidity: solana_pubkey::Pubkey,
    pub placeholder_user_destination_collateral: solana_pubkey::Pubkey,
    pub collateral_token_program: solana_pubkey::Pubkey,
    pub liquidity_token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositReserveLiquidityAndObligationCollateral {
    type ArrangedAccounts = DepositReserveLiquidityAndObligationCollateralInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, obligation, lending_market, lending_market_authority, reserve, reserve_liquidity_mint, reserve_liquidity_supply, reserve_collateral_mint, reserve_destination_deposit_collateral, user_source_liquidity, placeholder_user_destination_collateral, collateral_token_program, liquidity_token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            DepositReserveLiquidityAndObligationCollateralInstructionAccounts {
                owner: owner.pubkey,
                obligation: obligation.pubkey,
                lending_market: lending_market.pubkey,
                lending_market_authority: lending_market_authority.pubkey,
                reserve: reserve.pubkey,
                reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
                reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
                reserve_collateral_mint: reserve_collateral_mint.pubkey,
                reserve_destination_deposit_collateral: reserve_destination_deposit_collateral
                    .pubkey,
                user_source_liquidity: user_source_liquidity.pubkey,
                placeholder_user_destination_collateral: placeholder_user_destination_collateral
                    .pubkey,
                collateral_token_program: collateral_token_program.pubkey,
                liquidity_token_program: liquidity_token_program.pubkey,
                instruction_sysvar_account: instruction_sysvar_account.pubkey,
            },
        )
    }
}
