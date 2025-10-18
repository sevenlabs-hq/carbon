use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbafcef4656b46e5f")]
pub struct HandleReceiveFinalizedMessage {
    pub params: HandleReceiveMessageParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct HandleReceiveFinalizedMessageInstructionAccounts {
    pub authority_pda: solana_pubkey::Pubkey,
    pub token_messenger: solana_pubkey::Pubkey,
    pub remote_token_messenger: solana_pubkey::Pubkey,
    pub token_minter: solana_pubkey::Pubkey,
    pub local_token: solana_pubkey::Pubkey,
    pub token_pair: solana_pubkey::Pubkey,
    pub fee_recipient_token_account: solana_pubkey::Pubkey,
    pub recipient_token_account: solana_pubkey::Pubkey,
    pub custody_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for HandleReceiveFinalizedMessage {
    type ArrangedAccounts = HandleReceiveFinalizedMessageInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority_pda = next_account(&mut iter)?;
        let token_messenger = next_account(&mut iter)?;
        let remote_token_messenger = next_account(&mut iter)?;
        let token_minter = next_account(&mut iter)?;
        let local_token = next_account(&mut iter)?;
        let token_pair = next_account(&mut iter)?;
        let fee_recipient_token_account = next_account(&mut iter)?;
        let recipient_token_account = next_account(&mut iter)?;
        let custody_token_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(HandleReceiveFinalizedMessageInstructionAccounts {
            authority_pda,
            token_messenger,
            remote_token_messenger,
            token_minter,
            local_token,
            token_pair,
            fee_recipient_token_account,
            recipient_token_account,
            custody_token_account,
            token_program,
            event_authority,
            program,
        })
    }
}
