use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2f35192f6d7a1616")]
pub struct DepositInsuranceVault {
    pub amount: u64,
}

pub struct DepositInsuranceVaultInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub insurance_vault: solana_sdk::pubkey::Pubkey,
    pub insurance_deposit_account: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub zeta_vault: solana_sdk::pubkey::Pubkey,
    pub socialized_loss_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositInsuranceVault {
    type ArrangedAccounts = DepositInsuranceVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, insurance_vault, insurance_deposit_account, user_token_account, zeta_vault, socialized_loss_account, authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInsuranceVaultInstructionAccounts {
            state: state.pubkey,
            insurance_vault: insurance_vault.pubkey,
            insurance_deposit_account: insurance_deposit_account.pubkey,
            user_token_account: user_token_account.pubkey,
            zeta_vault: zeta_vault.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            authority: authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
