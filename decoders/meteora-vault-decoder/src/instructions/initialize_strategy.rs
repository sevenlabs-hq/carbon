use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd0779091b23969fc")]
pub struct InitializeStrategy {
    pub bumps: StrategyBumps,
    pub strategy_type: StrategyType,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeStrategyInstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub strategy_program: solana_pubkey::Pubkey,
    pub strategy: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub collateral_vault: solana_pubkey::Pubkey,
    pub collateral_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeStrategy {
    type ArrangedAccounts = InitializeStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let vault = next_account(&mut iter)?;
        let strategy_program = next_account(&mut iter)?;
        let strategy = next_account(&mut iter)?;
        let reserve = next_account(&mut iter)?;
        let collateral_vault = next_account(&mut iter)?;
        let collateral_mint = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(InitializeStrategyInstructionAccounts {
            vault,
            strategy_program,
            strategy,
            reserve,
            collateral_vault,
            collateral_mint,
            admin,
            system_program,
            rent,
            token_program,
        })
    }
}
