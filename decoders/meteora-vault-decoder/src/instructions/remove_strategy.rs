use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb9ee215b86d2611a")]
pub struct RemoveStrategy {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveStrategyInstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub strategy: solana_pubkey::Pubkey,
    pub strategy_program: solana_pubkey::Pubkey,
    pub collateral_vault: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveStrategy {
    type ArrangedAccounts = RemoveStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let vault = next_account(&mut iter)?;
        let strategy = next_account(&mut iter)?;
        let strategy_program = next_account(&mut iter)?;
        let collateral_vault = next_account(&mut iter)?;
        let reserve = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let fee_vault = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;

        Some(RemoveStrategyInstructionAccounts {
            vault,
            strategy,
            strategy_program,
            collateral_vault,
            reserve,
            token_vault,
            fee_vault,
            lp_mint,
            token_program,
            admin,
        })
    }
}
