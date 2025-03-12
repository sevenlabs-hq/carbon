

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x24484a13d2d2c0c0")]
pub struct LendingAccountWithdraw{
    pub amount: u64,
    pub withdraw_all: Option<bool>,
}

pub struct LendingAccountWithdrawInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub marginfi_account: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub bank: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub bank_liquidity_vault_authority: solana_sdk::pubkey::Pubkey,
    pub bank_liquidity_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountWithdraw {
    type ArrangedAccounts = LendingAccountWithdrawInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            marginfi_account,
            signer,
            bank,
            destination_token_account,
            bank_liquidity_vault_authority,
            bank_liquidity_vault,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountWithdrawInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
            bank: bank.pubkey,
            destination_token_account: destination_token_account.pubkey,
            bank_liquidity_vault_authority: bank_liquidity_vault_authority.pubkey,
            bank_liquidity_vault: bank_liquidity_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}