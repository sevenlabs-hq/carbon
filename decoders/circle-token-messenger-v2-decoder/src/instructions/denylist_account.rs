use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6574c57051f94bc2")]
pub struct DenylistAccount {
    pub params: DenylistParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DenylistAccountInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub denylister: solana_pubkey::Pubkey,
    pub token_messenger: solana_pubkey::Pubkey,
    pub denylist_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DenylistAccount {
    type ArrangedAccounts = DenylistAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let denylister = next_account(&mut iter)?;
        let token_messenger = next_account(&mut iter)?;
        let denylist_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(DenylistAccountInstructionAccounts {
            payer,
            denylister,
            token_messenger,
            denylist_account,
            system_program,
            event_authority,
            program,
        })
    }
}
