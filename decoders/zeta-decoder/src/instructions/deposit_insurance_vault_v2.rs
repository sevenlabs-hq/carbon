use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf22c18135b3b07c9")]
pub struct DepositInsuranceVaultV2 {
    pub amount: u64,
}

pub struct DepositInsuranceVaultV2InstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub insurance_deposit_account: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub zeta_vault: solana_pubkey::Pubkey,
    pub socialized_loss_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositInsuranceVaultV2 {
    type ArrangedAccounts = DepositInsuranceVaultV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, insurance_vault, insurance_deposit_account, user_token_account, zeta_vault, socialized_loss_account, authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInsuranceVaultV2InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
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
