use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x28")]
pub struct TransferOutOfEscrow {
    pub transfer_out_of_escrow_args: TransferOutOfEscrowArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferOutOfEscrowInstructionAccounts {
    pub escrow: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub attribute_mint: solana_pubkey::Pubkey,
    pub attribute_src: solana_pubkey::Pubkey,
    pub attribute_dst: solana_pubkey::Pubkey,
    pub escrow_mint: solana_pubkey::Pubkey,
    pub escrow_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub authority: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferOutOfEscrow {
    type ArrangedAccounts = TransferOutOfEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let escrow = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let attribute_mint = next_account(&mut iter)?;
        let attribute_src = next_account(&mut iter)?;
        let attribute_dst = next_account(&mut iter)?;
        let escrow_mint = next_account(&mut iter)?;
        let escrow_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;
        let authority = next_account(&mut iter);

        Some(TransferOutOfEscrowInstructionAccounts {
            escrow,
            metadata,
            payer,
            attribute_mint,
            attribute_src,
            attribute_dst,
            escrow_mint,
            escrow_account,
            system_program,
            ata_program,
            token_program,
            sysvar_instructions,
            authority,
        })
    }
}
