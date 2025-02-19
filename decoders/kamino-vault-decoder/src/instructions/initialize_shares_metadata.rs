use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x030fac72c8008320")]
pub struct InitializeSharesMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct InitializeSharesMetadataInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub shares_metadata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeSharesMetadata {
    type ArrangedAccounts = InitializeSharesMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, vault_state, shares_mint, base_vault_authority, shares_metadata, system_program, rent, metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeSharesMetadataInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            shares_mint: shares_mint.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            shares_metadata: shares_metadata.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}
