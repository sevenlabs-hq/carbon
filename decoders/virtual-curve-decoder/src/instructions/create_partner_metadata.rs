use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc0a8eabfbce2e3ff")]
pub struct CreatePartnerMetadata {
    pub metadata: CreatePartnerMetadataParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreatePartnerMetadataInstructionAccounts {
    pub partner_metadata: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePartnerMetadata {
    type ArrangedAccounts = CreatePartnerMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [partner_metadata, payer, fee_claimer, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreatePartnerMetadataInstructionAccounts {
            partner_metadata: partner_metadata.pubkey,
            payer: payer.pubkey,
            fee_claimer: fee_claimer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
