use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x96564774a75d0e68")]
pub struct RouteWithTokenLedger {
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct RouteWithTokenLedgerInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub user_source_token_account: solana_sdk::pubkey::Pubkey,
    pub user_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub platform_fee_account: solana_sdk::pubkey::Pubkey,
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RouteWithTokenLedger {
    type ArrangedAccounts = RouteWithTokenLedgerInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let user_transfer_authority = accounts.get(1)?;
        let user_source_token_account = accounts.get(2)?;
        let user_destination_token_account = accounts.get(3)?;
        let destination_token_account = accounts.get(4)?;
        let destination_mint = accounts.get(5)?;
        let platform_fee_account = accounts.get(6)?;
        let token_ledger = accounts.get(7)?;
        let event_authority = accounts.get(8)?;
        let program = accounts.get(9)?;

        Some(RouteWithTokenLedgerInstructionAccounts {
            token_program: token_program.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            user_source_token_account: user_source_token_account.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            destination_mint: destination_mint.pubkey,
            platform_fee_account: platform_fee_account.pubkey,
            token_ledger: token_ledger.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
