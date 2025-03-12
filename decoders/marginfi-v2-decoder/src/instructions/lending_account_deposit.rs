

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xab5eeb675240d48c")]
pub struct LendingAccountDeposit{
    pub amount: u64,
}

pub struct LendingAccountDepositInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub marginfi_account: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub bank: solana_sdk::pubkey::Pubkey,
    pub signer_token_account: solana_sdk::pubkey::Pubkey,
    pub bank_liquidity_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountDeposit {
    type ArrangedAccounts = LendingAccountDepositInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            marginfi_account,
            signer,
            bank,
            signer_token_account,
            bank_liquidity_vault,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LendingAccountDepositInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
            bank: bank.pubkey,
            signer_token_account: signer_token_account.pubkey,
            bank_liquidity_vault: bank_liquidity_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}