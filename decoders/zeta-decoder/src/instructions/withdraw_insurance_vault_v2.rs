use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcb472c94e0f245a5")]
pub struct WithdrawInsuranceVaultV2 {
    pub percentage_amount: u64,
}

pub struct WithdrawInsuranceVaultV2InstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub insurance_vault: solana_sdk::pubkey::Pubkey,
    pub insurance_deposit_account: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawInsuranceVaultV2 {
    type ArrangedAccounts = WithdrawInsuranceVaultV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, insurance_vault, insurance_deposit_account, user_token_account, authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawInsuranceVaultV2InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            insurance_vault: insurance_vault.pubkey,
            insurance_deposit_account: insurance_deposit_account.pubkey,
            user_token_account: user_token_account.pubkey,
            authority: authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
