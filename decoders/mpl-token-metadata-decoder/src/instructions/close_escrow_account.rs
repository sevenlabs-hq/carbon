

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x27")]
pub struct CloseEscrowAccount{
}

pub struct CloseEscrowAccountInstructionAccounts {
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseEscrowAccount {
    type ArrangedAccounts = CloseEscrowAccountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let escrow = accounts.get(0)?;
        let metadata = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let token_account = accounts.get(3)?;
        let edition = accounts.get(4)?;
        let payer = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let sysvar_instructions = accounts.get(7)?;

        Some(CloseEscrowAccountInstructionAccounts {
            escrow: escrow.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            token_account: token_account.pubkey,
            edition: edition.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
        })
    }
}