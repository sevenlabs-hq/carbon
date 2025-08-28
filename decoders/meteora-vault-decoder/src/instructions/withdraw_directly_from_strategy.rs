use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc98d922ead74c616")]
pub struct WithdrawDirectlyFromStrategy {
    pub unmint_amount: u64,
    pub min_out_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WithdrawDirectlyFromStrategyInstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub strategy: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub strategy_program: solana_pubkey::Pubkey,
    pub collateral_vault: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub user_token: solana_pubkey::Pubkey,
    pub user_lp: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawDirectlyFromStrategy {
    type ArrangedAccounts = WithdrawDirectlyFromStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let vault = next_account(&mut iter)?;
        let strategy = next_account(&mut iter)?;
        let reserve = next_account(&mut iter)?;
        let strategy_program = next_account(&mut iter)?;
        let collateral_vault = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let fee_vault = next_account(&mut iter)?;
        let user_token = next_account(&mut iter)?;
        let user_lp = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(WithdrawDirectlyFromStrategyInstructionAccounts {
            vault,
            strategy,
            reserve,
            strategy_program,
            collateral_vault,
            token_vault,
            lp_mint,
            fee_vault,
            user_token,
            user_lp,
            user,
            token_program,
        })
    }
}
