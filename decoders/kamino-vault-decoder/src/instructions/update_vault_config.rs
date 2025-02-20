use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7a0315de9effee9d")]
pub struct UpdateVaultConfig {
    pub entry: VaultConfigField,
    pub data: Vec<u8>,
}

pub struct UpdateVaultConfigInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub klend_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateVaultConfig {
    type ArrangedAccounts = UpdateVaultConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, vault_state, klend_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateVaultConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            klend_program: klend_program.pubkey,
        })
    }
}
