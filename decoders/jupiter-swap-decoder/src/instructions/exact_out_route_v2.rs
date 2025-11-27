use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9d8ab85215f4f324")]
pub struct ExactOutRouteV2 {
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u16,
    pub positive_slippage_bps: u16,
    pub route_plan: Vec<RoutePlanStepV2>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ExactOutRouteV2InstructionAccounts {
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub user_source_token_account: solana_pubkey::Pubkey,
    pub user_destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub source_token_program: solana_pubkey::Pubkey,
    pub destination_token_program: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExactOutRouteV2 {
    type ArrangedAccounts = ExactOutRouteV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user_transfer_authority = next_account(&mut iter)?;
        let user_source_token_account = next_account(&mut iter)?;
        let user_destination_token_account = next_account(&mut iter)?;
        let source_mint = next_account(&mut iter)?;
        let destination_mint = next_account(&mut iter)?;
        let source_token_program = next_account(&mut iter)?;
        let destination_token_program = next_account(&mut iter)?;
        let destination_token_account = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(ExactOutRouteV2InstructionAccounts {
            user_transfer_authority,
            user_source_token_account,
            user_destination_token_account,
            source_mint,
            destination_mint,
            source_token_program,
            destination_token_program,
            destination_token_account,
            event_authority,
            program,
        })
    }
}
