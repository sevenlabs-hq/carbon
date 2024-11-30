
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2e")]
pub struct Lock{
    pub lock_args: LockArgs,
}

pub struct LockInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token_owner: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Lock {
    type ArrangedAccounts = LockInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let token_owner = accounts.get(1)?;
        let token = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let metadata = accounts.get(4)?;
        let edition = accounts.get(5)?;
        let token_record = accounts.get(6)?;
        let payer = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let sysvar_instructions = accounts.get(9)?;
        let spl_token_program = accounts.get(10)?;
        let authorization_rules_program = accounts.get(11)?;
        let authorization_rules = accounts.get(12)?;

        Some(LockInstructionAccounts {
            authority: authority.pubkey,
            token_owner: token_owner.pubkey,
            token: token.pubkey,
            mint: mint.pubkey,
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            token_record: token_record.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}