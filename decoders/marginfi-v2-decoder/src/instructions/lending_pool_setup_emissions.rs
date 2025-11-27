use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xce6178ac71cca946")]
pub struct LendingPoolSetupEmissions {
    pub flags: u64,
    pub rate: u64,
    pub total_emissions: u64,
}

pub struct LendingPoolSetupEmissionsInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub emissions_mint: solana_pubkey::Pubkey,
    pub emissions_auth: solana_pubkey::Pubkey,
    pub emissions_token_account: solana_pubkey::Pubkey,
    pub emissions_funding_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingPoolSetupEmissions {
    type ArrangedAccounts = LendingPoolSetupEmissionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            marginfi_group,
            admin,
            bank,
            emissions_mint,
            emissions_auth,
            emissions_token_account,
            emissions_funding_account,
            token_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(LendingPoolSetupEmissionsInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            bank: bank.pubkey,
            emissions_mint: emissions_mint.pubkey,
            emissions_auth: emissions_auth.pubkey,
            emissions_token_account: emissions_token_account.pubkey,
            emissions_funding_account: emissions_funding_account.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
