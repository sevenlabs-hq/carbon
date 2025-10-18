use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6780de8672c816c8")]
pub struct CollectProtocolFeesV2 {
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolFeesV2InstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub token_destination_a: solana_pubkey::Pubkey,
    pub token_destination_b: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectProtocolFeesV2 {
    type ArrangedAccounts = CollectProtocolFeesV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let collect_protocol_fees_authority = next_account(&mut iter)?;
        let token_mint_a = next_account(&mut iter)?;
        let token_mint_b = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let token_destination_a = next_account(&mut iter)?;
        let token_destination_b = next_account(&mut iter)?;
        let token_program_a = next_account(&mut iter)?;
        let token_program_b = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;

        Some(CollectProtocolFeesV2InstructionAccounts {
            whirlpools_config,
            whirlpool,
            collect_protocol_fees_authority,
            token_mint_a,
            token_mint_b,
            token_vault_a,
            token_vault_b,
            token_destination_a,
            token_destination_b,
            token_program_a,
            token_program_b,
            memo_program,
        })
    }
}
