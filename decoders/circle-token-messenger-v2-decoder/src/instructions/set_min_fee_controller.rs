use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc38e4a54ea5eb471")]
pub struct SetMinFeeController {
    pub params: SetMinFeeControllerParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetMinFeeControllerInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub token_messenger: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetMinFeeController {
    type ArrangedAccounts = SetMinFeeControllerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let token_messenger = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SetMinFeeControllerInstructionAccounts {
            owner,
            token_messenger,
            event_authority,
            program,
        })
    }
}
