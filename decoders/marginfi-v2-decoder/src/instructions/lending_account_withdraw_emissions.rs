use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xea1654d676b08caa")]
pub struct LendingAccountWithdrawEmissions {}

pub struct LendingAccountWithdrawEmissionsInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub emissions_mint: solana_pubkey::Pubkey,
    pub emissions_auth: solana_pubkey::Pubkey,
    pub emissions_vault: solana_pubkey::Pubkey,
    pub destination_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountWithdrawEmissions {
    type ArrangedAccounts = LendingAccountWithdrawEmissionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            marginfi_account,
            signer,
            bank,
            emissions_mint,
            emissions_auth,
            emissions_vault,
            destination_account,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(LendingAccountWithdrawEmissionsInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
            bank: bank.pubkey,
            emissions_mint: emissions_mint.pubkey,
            emissions_auth: emissions_auth.pubkey,
            emissions_vault: emissions_vault.pubkey,
            destination_account: destination_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
