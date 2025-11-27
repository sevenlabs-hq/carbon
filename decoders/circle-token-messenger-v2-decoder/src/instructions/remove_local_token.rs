use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b2b42aabc2c6d61")]
pub struct RemoveLocalToken {
    pub params: RemoveLocalTokenParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveLocalTokenInstructionAccounts {
    pub payee: solana_pubkey::Pubkey,
    pub token_controller: solana_pubkey::Pubkey,
    pub token_minter: solana_pubkey::Pubkey,
    pub local_token: solana_pubkey::Pubkey,
    pub custody_token_account: solana_pubkey::Pubkey,
    pub custody_token_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveLocalToken {
    type ArrangedAccounts = RemoveLocalTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payee = next_account(&mut iter)?;
        let token_controller = next_account(&mut iter)?;
        let token_minter = next_account(&mut iter)?;
        let local_token = next_account(&mut iter)?;
        let custody_token_account = next_account(&mut iter)?;
        let custody_token_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(RemoveLocalTokenInstructionAccounts {
            payee,
            token_controller,
            token_minter,
            local_token,
            custody_token_account,
            custody_token_mint,
            token_program,
            event_authority,
            program,
        })
    }
}
