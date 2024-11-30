

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x30")]
pub struct Migrate{
}

pub struct MigrateInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub token_owner: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub delegate_record: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Migrate {
    type ArrangedAccounts = MigrateInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let edition = accounts.get(1)?;
        let token = accounts.get(2)?;
        let token_owner = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let payer = accounts.get(5)?;
        let authority = accounts.get(6)?;
        let collection_metadata = accounts.get(7)?;
        let delegate_record = accounts.get(8)?;
        let token_record = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let sysvar_instructions = accounts.get(11)?;
        let spl_token_program = accounts.get(12)?;
        let authorization_rules_program = accounts.get(13)?;
        let authorization_rules = accounts.get(14)?;

        Some(MigrateInstructionAccounts {
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            token: token.pubkey,
            token_owner: token_owner.pubkey,
            mint: mint.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            collection_metadata: collection_metadata.pubkey,
            delegate_record: delegate_record.pubkey,
            token_record: token_record.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}