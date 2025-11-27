use carbon_core::account_utils::next_account;
use carbon_core::{borsh, CarbonDeserialize};
use serde::{Deserialize, Serialize};

#[derive(CarbonDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x38fc74089edfcd5f")]
pub struct BuyExactSolIn {
    pub spendable_sol_in: u64,
    pub min_tokens_out: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuyExactSolInInstructionAccounts {
    pub global: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub associated_bonding_curve: solana_pubkey::Pubkey,
    pub associated_user: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub creator_vault: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
    pub global_volume_accumulator: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
    pub fee_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyExactSolIn {
    type ArrangedAccounts = BuyExactSolInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let global = next_account(&mut iter)?;
        let fee_recipient = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_bonding_curve = next_account(&mut iter)?;
        let associated_user = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let creator_vault = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let fee_config = next_account(&mut iter)?;
        let fee_program = next_account(&mut iter)?;

        Some(BuyExactSolInInstructionAccounts {
            global,
            fee_recipient,
            mint,
            bonding_curve,
            associated_bonding_curve,
            associated_user,
            user,
            system_program,
            token_program,
            creator_vault,
            event_authority,
            program,
            global_volume_accumulator,
            user_volume_accumulator,
            fee_config,
            fee_program,
        })
    }
}
