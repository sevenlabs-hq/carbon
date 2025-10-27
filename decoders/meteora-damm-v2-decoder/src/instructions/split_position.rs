use crate::types::SplitPositionParameters;
use carbon_core::deserialize::ArrangeAccounts;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xacf1dd8aa11dfd2a")]
pub struct SplitPosition {
    pub params: SplitPositionParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SplitPositionInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub first_position: solana_pubkey::Pubkey,
    pub first_position_nft_account: solana_pubkey::Pubkey,
    pub second_position: solana_pubkey::Pubkey,
    pub second_position_nft_account: solana_pubkey::Pubkey,
    pub first_owner: solana_pubkey::Pubkey,
    pub second_owner: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl ArrangeAccounts for SplitPosition {
    type ArrangedAccounts = SplitPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, first_position, first_position_nft_account, second_position, second_position_nft_account, first_owner, second_owner, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SplitPositionInstructionAccounts {
            pool: pool.pubkey,
            first_position: first_position.pubkey,
            first_position_nft_account: first_position_nft_account.pubkey,
            second_position: second_position.pubkey,
            second_position_nft_account: second_position_nft_account.pubkey,
            first_owner: first_owner.pubkey,
            second_owner: second_owner.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
