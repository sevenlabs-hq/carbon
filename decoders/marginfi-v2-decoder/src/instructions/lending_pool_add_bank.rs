
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd744484ed0da67b6")]
pub struct LendingPoolAddBank{
    pub bank_config: BankConfigCompact,
}

pub struct LendingPoolAddBankInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub bank_mint: solana_sdk::pubkey::Pubkey,
    pub bank: solana_sdk::pubkey::Pubkey,
    pub liquidity_vault_authority: solana_sdk::pubkey::Pubkey,
    pub liquidity_vault: solana_sdk::pubkey::Pubkey,
    pub insurance_vault_authority: solana_sdk::pubkey::Pubkey,
    pub insurance_vault: solana_sdk::pubkey::Pubkey,
    pub fee_vault_authority: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolAddBank {
    type ArrangedAccounts = LendingPoolAddBankInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            admin,
            fee_payer,
            bank_mint,
            bank,
            liquidity_vault_authority,
            liquidity_vault,
            insurance_vault_authority,
            insurance_vault,
            fee_vault_authority,
            fee_vault,
            rent,
            token_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingPoolAddBankInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            fee_payer: fee_payer.pubkey,
            bank_mint: bank_mint.pubkey,
            bank: bank.pubkey,
            liquidity_vault_authority: liquidity_vault_authority.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            insurance_vault_authority: insurance_vault_authority.pubkey,
            insurance_vault: insurance_vault.pubkey,
            fee_vault_authority: fee_vault_authority.pubkey,
            fee_vault: fee_vault.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}