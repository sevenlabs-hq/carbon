use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0bc442eb3beddf6f")]
pub struct RebalanceInsuranceVault {}

pub struct RebalanceInsuranceVaultInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_vault: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub treasury_wallet: solana_pubkey::Pubkey,
    pub socialized_loss_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RebalanceInsuranceVault {
    type ArrangedAccounts = RebalanceInsuranceVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_vault, insurance_vault, treasury_wallet, socialized_loss_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RebalanceInsuranceVaultInstructionAccounts {
            state: state.pubkey,
            zeta_vault: zeta_vault.pubkey,
            insurance_vault: insurance_vault.pubkey,
            treasury_wallet: treasury_wallet.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
