use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb8eefe5ca4c7c967")]
pub struct RebalanceInsuranceVaultV2 {}

pub struct RebalanceInsuranceVaultV2InstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub zeta_vault: solana_sdk::pubkey::Pubkey,
    pub insurance_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_wallet: solana_sdk::pubkey::Pubkey,
    pub treasury_split_token_account: solana_sdk::pubkey::Pubkey,
    pub socialized_loss_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RebalanceInsuranceVaultV2 {
    type ArrangedAccounts = RebalanceInsuranceVaultV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_vault, insurance_vault, treasury_wallet, treasury_split_token_account, socialized_loss_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RebalanceInsuranceVaultV2InstructionAccounts {
            state: state.pubkey,
            zeta_vault: zeta_vault.pubkey,
            insurance_vault: insurance_vault.pubkey,
            treasury_wallet: treasury_wallet.pubkey,
            treasury_split_token_account: treasury_split_token_account.pubkey,
            socialized_loss_account: socialized_loss_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
