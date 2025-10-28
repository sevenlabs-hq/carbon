use carbon_core::deserialize::ArrangeAccounts;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdd93e4cf8cd41177")]
pub struct SplitPosition2 {
    pub numerator: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SplitPosition2InstructionAccounts {
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

impl ArrangeAccounts for SplitPosition2 {
    type ArrangedAccounts = SplitPosition2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, first_position, first_position_nft_account, second_position, second_position_nft_account, first_owner, second_owner, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SplitPosition2InstructionAccounts {
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
