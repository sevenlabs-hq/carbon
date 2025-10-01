use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x72c6230329c4c2f6")]
pub struct SetMinFee {
    pub params: SetMinFeeParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetMinFeeInstructionAccounts {
    pub min_fee_controller: solana_pubkey::Pubkey,
    pub token_messenger: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetMinFee {
    type ArrangedAccounts = SetMinFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let min_fee_controller = next_account(&mut iter)?;
        let token_messenger = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SetMinFeeInstructionAccounts {
            min_fee_controller,
            token_messenger,
            event_authority,
            program,
        })
    }
}
