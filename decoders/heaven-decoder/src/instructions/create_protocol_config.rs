use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x7372186f0e3571fe")]
pub struct CreateProtocolConfig {
    pub version: u16,
    pub params: ProtocolConfigParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateProtocolConfigInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub protocol_config_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
    pub msol_token_vault: solana_pubkey::Pubkey,
    pub msol_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateProtocolConfig {
    type ArrangedAccounts = CreateProtocolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, associated_token_program, payer, owner, protocol_config_state, system_program, protocol_owner_state, msol_token_vault, msol_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateProtocolConfigInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            system_program: system_program.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            msol_token_vault: msol_token_vault.pubkey,
            msol_mint: msol_mint.pubkey,
        })
    }
}
