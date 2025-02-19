use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x28")]
pub struct TransferOutOfEscrow {
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

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [escrow, metadata, payer, attribute_mint, attribute_src, attribute_dst, escrow_mint, escrow_account, system_program, ata_program, token_program, sysvar_instructions, authority] =
            accounts
        else {
            return None;
        };

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
