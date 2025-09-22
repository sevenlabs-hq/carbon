use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
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
        let mut iter = accounts.iter();
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let protocol_config_state = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let protocol_owner_state = next_account(&mut iter)?;
        let msol_token_vault = next_account(&mut iter)?;
        let msol_mint = next_account(&mut iter)?;

        Some(CreateProtocolConfigInstructionAccounts {
            token_program,
            associated_token_program,
            payer,
            owner,
            protocol_config_state,
            system_program,
            protocol_owner_state,
            msol_token_vault,
            msol_mint,
        })
    }
}
