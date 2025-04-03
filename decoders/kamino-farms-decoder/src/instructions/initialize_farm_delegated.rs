use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfa546519334dcc5b")]
pub struct InitializeFarmDelegated {}

pub struct InitializeFarmDelegatedInstructionAccounts {
    pub farm_admin: solana_pubkey::Pubkey,
    pub farm_delegate: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub farm_vaults_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeFarmDelegated {
    type ArrangedAccounts = InitializeFarmDelegatedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [farm_admin, farm_delegate, farm_state, global_config, farm_vaults_authority, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeFarmDelegatedInstructionAccounts {
            farm_admin: farm_admin.pubkey,
            farm_delegate: farm_delegate.pubkey,
            farm_state: farm_state.pubkey,
            global_config: global_config.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
