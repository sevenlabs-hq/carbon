
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x28")]
pub struct TransferOutOfEscrow{
    pub transfer_out_of_escrow_args: TransferOutOfEscrowArgs,
}

pub struct TransferOutOfEscrowInstructionAccounts {
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub attribute_mint: solana_sdk::pubkey::Pubkey,
    pub attribute_src: solana_sdk::pubkey::Pubkey,
    pub attribute_dst: solana_sdk::pubkey::Pubkey,
    pub escrow_mint: solana_sdk::pubkey::Pubkey,
    pub escrow_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub ata_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferOutOfEscrow {
    type ArrangedAccounts = TransferOutOfEscrowInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let escrow = accounts.get(0)?;
        let metadata = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let attribute_mint = accounts.get(3)?;
        let attribute_src = accounts.get(4)?;
        let attribute_dst = accounts.get(5)?;
        let escrow_mint = accounts.get(6)?;
        let escrow_account = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let ata_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let sysvar_instructions = accounts.get(11)?;
        let authority = accounts.get(12)?;

        Some(TransferOutOfEscrowInstructionAccounts {
            escrow: escrow.pubkey,
            metadata: metadata.pubkey,
            payer: payer.pubkey,
            attribute_mint: attribute_mint.pubkey,
            attribute_src: attribute_src.pubkey,
            attribute_dst: attribute_dst.pubkey,
            escrow_mint: escrow_mint.pubkey,
            escrow_account: escrow_account.pubkey,
            system_program: system_program.pubkey,
            ata_program: ata_program.pubkey,
            token_program: token_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            authority: authority.pubkey,
        })
    }
}