use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6ff53e83cc6cdf9b")]
pub struct DepositForBurnWithHook {
    pub params: DepositForBurnWithHookParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositForBurnWithHookInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub event_rent_payer: solana_pubkey::Pubkey,
    pub sender_authority_pda: solana_pubkey::Pubkey,
    pub burn_token_account: solana_pubkey::Pubkey,
    pub denylist_account: solana_pubkey::Pubkey,
    pub message_transmitter: solana_pubkey::Pubkey,
    pub token_messenger: solana_pubkey::Pubkey,
    pub remote_token_messenger: solana_pubkey::Pubkey,
    pub token_minter: solana_pubkey::Pubkey,
    pub local_token: solana_pubkey::Pubkey,
    pub burn_token_mint: solana_pubkey::Pubkey,
    pub message_sent_event_data: solana_pubkey::Pubkey,
    pub message_transmitter_program: solana_pubkey::Pubkey,
    pub token_messenger_minter_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositForBurnWithHook {
    type ArrangedAccounts = DepositForBurnWithHookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let event_rent_payer = next_account(&mut iter)?;
        let sender_authority_pda = next_account(&mut iter)?;
        let burn_token_account = next_account(&mut iter)?;
        let denylist_account = next_account(&mut iter)?;
        let message_transmitter = next_account(&mut iter)?;
        let token_messenger = next_account(&mut iter)?;
        let remote_token_messenger = next_account(&mut iter)?;
        let token_minter = next_account(&mut iter)?;
        let local_token = next_account(&mut iter)?;
        let burn_token_mint = next_account(&mut iter)?;
        let message_sent_event_data = next_account(&mut iter)?;
        let message_transmitter_program = next_account(&mut iter)?;
        let token_messenger_minter_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(DepositForBurnWithHookInstructionAccounts {
            owner,
            event_rent_payer,
            sender_authority_pda,
            burn_token_account,
            denylist_account,
            message_transmitter,
            token_messenger,
            remote_token_messenger,
            token_minter,
            local_token,
            burn_token_mint,
            message_sent_event_data,
            message_transmitter_program,
            token_messenger_minter_program,
            token_program,
            system_program,
            event_authority,
            program,
        })
    }
}
