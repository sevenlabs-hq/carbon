

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x030fac72c8008320")]
pub struct InitializeSharesMetadata{
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

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let vault_state = accounts.get(1)?;
        let shares_mint = accounts.get(2)?;
        let base_vault_authority = accounts.get(3)?;
        let shares_metadata = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let rent = accounts.get(6)?;
        let metadata_program = accounts.get(7)?;

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