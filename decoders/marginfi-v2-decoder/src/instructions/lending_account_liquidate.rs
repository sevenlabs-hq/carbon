use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd6a997d5fba756db")]
pub struct LendingAccountLiquidate {
    pub asset_amount: u64,
}

pub struct LendingAccountLiquidateInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub asset_bank: solana_pubkey::Pubkey,
    pub liab_bank: solana_pubkey::Pubkey,
    pub liquidator_marginfi_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub liquidatee_marginfi_account: solana_pubkey::Pubkey,
    pub bank_liquidity_vault_authority: solana_pubkey::Pubkey,
    pub bank_liquidity_vault: solana_pubkey::Pubkey,
    pub bank_insurance_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountLiquidate {
    type ArrangedAccounts = LendingAccountLiquidateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            asset_bank,
            liab_bank,
            liquidator_marginfi_account,
            signer,
            liquidatee_marginfi_account,
            bank_liquidity_vault_authority,
            bank_liquidity_vault,
            bank_insurance_vault,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(LendingAccountLiquidateInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            asset_bank: asset_bank.pubkey,
            liab_bank: liab_bank.pubkey,
            liquidator_marginfi_account: liquidator_marginfi_account.pubkey,
            signer: signer.pubkey,
            liquidatee_marginfi_account: liquidatee_marginfi_account.pubkey,
            bank_liquidity_vault_authority: bank_liquidity_vault_authority.pubkey,
            bank_liquidity_vault: bank_liquidity_vault.pubkey,
            bank_insurance_vault: bank_insurance_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
