use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa20b388b5a8046ad")]
pub struct LendingPoolHandleBankruptcy {}

pub struct LendingPoolHandleBankruptcyInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub bank: solana_sdk::pubkey::Pubkey,
    pub marginfi_account: solana_sdk::pubkey::Pubkey,
    pub liquidity_vault: solana_sdk::pubkey::Pubkey,
    pub insurance_vault: solana_sdk::pubkey::Pubkey,
    pub insurance_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolHandleBankruptcy {
    type ArrangedAccounts = LendingPoolHandleBankruptcyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, admin, bank, marginfi_account, liquidity_vault, insurance_vault, insurance_vault_authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LendingPoolHandleBankruptcyInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            bank: bank.pubkey,
            marginfi_account: marginfi_account.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            insurance_vault: insurance_vault.pubkey,
            insurance_vault_authority: insurance_vault_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
