
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6780de8672c816c8")]
pub struct CollectProtocolFeesV2{
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

pub struct CollectProtocolFeesV2InstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub token_destination_a: solana_sdk::pubkey::Pubkey,
    pub token_destination_b: solana_sdk::pubkey::Pubkey,
    pub token_program_a: solana_sdk::pubkey::Pubkey,
    pub token_program_b: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CollectProtocolFeesV2 {
    type ArrangedAccounts = CollectProtocolFeesV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpool = accounts.get(1)?;
        let collect_protocol_fees_authority = accounts.get(2)?;
        let token_mint_a = accounts.get(3)?;
        let token_mint_b = accounts.get(4)?;
        let token_vault_a = accounts.get(5)?;
        let token_vault_b = accounts.get(6)?;
        let token_destination_a = accounts.get(7)?;
        let token_destination_b = accounts.get(8)?;
        let token_program_a = accounts.get(9)?;
        let token_program_b = accounts.get(10)?;
        let memo_program = accounts.get(11)?;

        Some(CollectProtocolFeesV2InstructionAccounts {
            whirlpools_config: *whirlpools_config,
            whirlpool: *whirlpool,
            collect_protocol_fees_authority: *collect_protocol_fees_authority,
            token_mint_a: *token_mint_a,
            token_mint_b: *token_mint_b,
            token_vault_a: *token_vault_a,
            token_vault_b: *token_vault_b,
            token_destination_a: *token_destination_a,
            token_destination_b: *token_destination_b,
            token_program_a: *token_program_a,
            token_program_b: *token_program_b,
            memo_program: *memo_program,
        })
    }
}