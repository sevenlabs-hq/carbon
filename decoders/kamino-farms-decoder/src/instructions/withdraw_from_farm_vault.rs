use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x165280fa564f7c4e")]
pub struct WithdrawFromFarmVault {
    pub amount: u64,
}

pub struct WithdrawFromFarmVaultInstructionAccounts {
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub withdrawer_token_account: solana_pubkey::Pubkey,
    pub farm_vault: solana_pubkey::Pubkey,
    pub farm_vaults_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFromFarmVault {
    type ArrangedAccounts = WithdrawFromFarmVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            withdraw_authority,
            farm_state,
            withdrawer_token_account,
            farm_vault,
            farm_vaults_authority,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(WithdrawFromFarmVaultInstructionAccounts {
            withdraw_authority: withdraw_authority.pubkey,
            farm_state: farm_state.pubkey,
            withdrawer_token_account: withdrawer_token_account.pubkey,
            farm_vault: farm_vault.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
