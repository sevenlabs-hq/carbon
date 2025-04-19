use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2f5e7e73dde2c285")]
pub struct MigrationMeteoraDammCreateMetadata {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrationMeteoraDammCreateMetadataInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrationMeteoraDammCreateMetadata {
    type ArrangedAccounts = MigrationMeteoraDammCreateMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, config, migration_metadata, payer, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrationMeteoraDammCreateMetadataInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            config: config.pubkey,
            migration_metadata: migration_metadata.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
