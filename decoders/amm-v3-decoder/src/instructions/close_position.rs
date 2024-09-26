
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x873e91a6bf3809fa")]
pub struct ClosePosition{
}

pub struct ClosePositionInstructionAccounts {
    pub nft_owner: solana_sdk::pubkey::Pubkey,
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub position_nft_account: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ClosePosition {
    type ArrangedAccounts = ClosePositionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let nft_owner = accounts.get(0)?;
        let position_nft_mint = accounts.get(1)?;
        let position_nft_account = accounts.get(2)?;
        let personal_position = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let token_program = accounts.get(5)?;

        Some(ClosePositionInstructionAccounts {
            nft_owner: *nft_owner,
            position_nft_mint: *position_nft_mint,
            position_nft_account: *position_nft_account,
            personal_position: *personal_position,
            system_program: *system_program,
            token_program: *token_program,
        })
    }
}