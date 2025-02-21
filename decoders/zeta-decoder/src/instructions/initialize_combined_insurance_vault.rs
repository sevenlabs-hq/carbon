use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4d12b590db54066a")]
pub struct InitializeCombinedInsuranceVault {
    pub nonce: u8,
}

pub struct InitializeCombinedInsuranceVaultInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub insurance_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub usdc_mint: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCombinedInsuranceVault {
    type ArrangedAccounts = InitializeCombinedInsuranceVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, insurance_vault, token_program, usdc_mint, admin, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeCombinedInsuranceVaultInstructionAccounts {
            state: state.pubkey,
            insurance_vault: insurance_vault.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
