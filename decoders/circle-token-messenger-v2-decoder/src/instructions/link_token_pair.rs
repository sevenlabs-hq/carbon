use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x44a218687d2e820c")]
pub struct LinkTokenPair {
    pub params: LinkTokenPairParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LinkTokenPairInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub token_controller: solana_pubkey::Pubkey,
    pub token_minter: solana_pubkey::Pubkey,
    pub token_pair: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LinkTokenPair {
    type ArrangedAccounts = LinkTokenPairInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let token_controller = next_account(&mut iter)?;
        let token_minter = next_account(&mut iter)?;
        let token_pair = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(LinkTokenPairInstructionAccounts {
            payer,
            token_controller,
            token_minter,
            token_pair,
            system_program,
            event_authority,
            program,
        })
    }
}
