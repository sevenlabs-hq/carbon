use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6cd1044815167685")]
pub struct DepositObligationCollateral {
    pub collateral_amount: u64,
}

pub struct DepositObligationCollateralInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub deposit_reserve: solana_pubkey::Pubkey,
    pub reserve_destination_collateral: solana_pubkey::Pubkey,
    pub user_source_collateral: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositObligationCollateral {
    type ArrangedAccounts = DepositObligationCollateralInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, obligation, lending_market, deposit_reserve, reserve_destination_collateral, user_source_collateral, token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositObligationCollateralInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            deposit_reserve: deposit_reserve.pubkey,
            reserve_destination_collateral: reserve_destination_collateral.pubkey,
            user_source_collateral: user_source_collateral.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
