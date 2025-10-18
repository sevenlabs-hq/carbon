use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x12")]
pub struct AuthorityConfigRevoke {
    pub account: solana_pubkey::Pubkey,
    pub privileges: Vec<Privilege>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AuthorityConfigRevokeInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub authority_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AuthorityConfigRevoke {
    type ArrangedAccounts = AuthorityConfigRevokeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let authority_config = next_account(&mut iter)?;

        Some(AuthorityConfigRevokeInstructionAccounts {
            authority,
            authority_config,
        })
    }
}
