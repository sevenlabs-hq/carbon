use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe6798f50779f6aaa")]
pub struct SharedAccountsRouteWithTokenLedger {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SharedAccountsRouteWithTokenLedgerInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub program_authority: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub source_token_account: solana_pubkey::Pubkey,
    pub program_source_token_account: solana_pubkey::Pubkey,
    pub program_destination_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub platform_fee_account: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub token_ledger: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SharedAccountsRouteWithTokenLedger {
    type ArrangedAccounts = SharedAccountsRouteWithTokenLedgerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_program = next_account(&mut iter)?;
        let program_authority = next_account(&mut iter)?;
        let user_transfer_authority = next_account(&mut iter)?;
        let source_token_account = next_account(&mut iter)?;
        let program_source_token_account = next_account(&mut iter)?;
        let program_destination_token_account = next_account(&mut iter)?;
        let destination_token_account = next_account(&mut iter)?;
        let source_mint = next_account(&mut iter)?;
        let destination_mint = next_account(&mut iter)?;
        let platform_fee_account = next_account(&mut iter)?;
        let token_2022_program = next_account(&mut iter)?;
        let token_ledger = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SharedAccountsRouteWithTokenLedgerInstructionAccounts {
            token_program,
            program_authority,
            user_transfer_authority,
            source_token_account,
            program_source_token_account,
            program_destination_token_account,
            destination_token_account,
            source_mint,
            destination_mint,
            platform_fee_account,
            token_2022_program,
            token_ledger,
            event_authority,
            program,
        })
    }
}
