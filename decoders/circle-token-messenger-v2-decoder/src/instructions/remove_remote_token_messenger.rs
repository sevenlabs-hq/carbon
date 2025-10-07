use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x41724255a962b192")]
pub struct RemoveRemoteTokenMessenger {
    pub params: RemoveRemoteTokenMessengerParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveRemoteTokenMessengerInstructionAccounts {
    pub payee: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_messenger: solana_pubkey::Pubkey,
    pub remote_token_messenger: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveRemoteTokenMessenger {
    type ArrangedAccounts = RemoveRemoteTokenMessengerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payee = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token_messenger = next_account(&mut iter)?;
        let remote_token_messenger = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(RemoveRemoteTokenMessengerInstructionAccounts {
            payee,
            owner,
            token_messenger,
            remote_token_messenger,
            event_authority,
            program,
        })
    }
}
