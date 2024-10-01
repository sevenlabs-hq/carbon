
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd033ef977b2bed5c")]
pub struct ExactOutRoute{
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct ExactOutRouteInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub user_source_token_account: solana_sdk::pubkey::Pubkey,
    pub user_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub platform_fee_account: solana_sdk::pubkey::Pubkey,
    pub token2022_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ExactOutRoute {
    type ArrangedAccounts = ExactOutRouteInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let user_transfer_authority = accounts.get(1)?;
        let user_source_token_account = accounts.get(2)?;
        let user_destination_token_account = accounts.get(3)?;
        let destination_token_account = accounts.get(4)?;
        let source_mint = accounts.get(5)?;
        let destination_mint = accounts.get(6)?;
        let platform_fee_account = accounts.get(7)?;
        let token2022_program = accounts.get(8)?;
        let event_authority = accounts.get(9)?;
        let program = accounts.get(10)?;

        Some(ExactOutRouteInstructionAccounts {
            token_program: *token_program,
            user_transfer_authority: *user_transfer_authority,
            user_source_token_account: *user_source_token_account,
            user_destination_token_account: *user_destination_token_account,
            destination_token_account: *destination_token_account,
            source_mint: *source_mint,
            destination_mint: *destination_mint,
            platform_fee_account: *platform_fee_account,
            token2022_program: *token2022_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}