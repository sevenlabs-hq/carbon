use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2574cd67f3c05cc6")]
pub struct WithdrawObligationCollateral {
    pub collateral_amount: u64,
}

pub struct WithdrawObligationCollateralInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub withdraw_reserve: solana_pubkey::Pubkey,
    pub reserve_source_collateral: solana_pubkey::Pubkey,
    pub user_destination_collateral: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawObligationCollateral {
    type ArrangedAccounts = WithdrawObligationCollateralInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, obligation, lending_market, lending_market_authority, withdraw_reserve, reserve_source_collateral, user_destination_collateral, token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawObligationCollateralInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            withdraw_reserve: withdraw_reserve.pubkey,
            reserve_source_collateral: reserve_source_collateral.pubkey,
            user_destination_collateral: user_destination_collateral.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
