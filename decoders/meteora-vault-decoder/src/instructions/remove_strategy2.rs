use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8a68d0947e23c30e")]
pub struct RemoveStrategy2 {
    pub max_admin_pay_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveStrategy2InstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub strategy: solana_pubkey::Pubkey,
    pub strategy_program: solana_pubkey::Pubkey,
    pub collateral_vault: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub token_admin_advance_payment: solana_pubkey::Pubkey,
    pub token_vault_advance_payment: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveStrategy2 {
    type ArrangedAccounts = RemoveStrategy2InstructionAccounts;

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
        let token_admin_advance_payment = next_account(&mut iter)?;
        let token_vault_advance_payment = next_account(&mut iter)?;
        let fee_vault = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;

        Some(RemoveStrategy2InstructionAccounts {
            vault,
            strategy,
            strategy_program,
            collateral_vault,
            reserve,
            token_vault,
            token_admin_advance_payment,
            token_vault_advance_payment,
            fee_vault,
            lp_mint,
            token_program,
            admin,
        })
    }
}
