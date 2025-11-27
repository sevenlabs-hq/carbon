use carbon_core::account_utils::next_account;
use carbon_core::{borsh, CarbonDeserialize};
use serde::{Deserialize, Serialize};

#[derive(CarbonDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc62e1552b4d9e870")]
pub struct BuyExactQuoteIn {
    pub spendable_quote_in: u64,
    pub min_base_amount_out: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuyExactQuoteInInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub user_base_token_account: solana_pubkey::Pubkey,
    pub user_quote_token_account: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub protocol_fee_recipient: solana_pubkey::Pubkey,
    pub protocol_fee_recipient_token_account: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
    pub coin_creator_vault_ata: solana_pubkey::Pubkey,
    pub coin_creator_vault_authority: solana_pubkey::Pubkey,
    pub global_volume_accumulator: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
    pub fee_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyExactQuoteIn {
    type ArrangedAccounts = BuyExactQuoteInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let user_base_token_account = next_account(&mut iter)?;
        let user_quote_token_account = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let protocol_fee_recipient = next_account(&mut iter)?;
        let protocol_fee_recipient_token_account = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;
        let coin_creator_vault_ata = next_account(&mut iter)?;
        let coin_creator_vault_authority = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let fee_config = next_account(&mut iter)?;
        let fee_program = next_account(&mut iter)?;

        Some(BuyExactQuoteInInstructionAccounts {
            pool,
            user,
            global_config,
            base_mint,
            quote_mint,
            user_base_token_account,
            user_quote_token_account,
            pool_base_token_account,
            pool_quote_token_account,
            protocol_fee_recipient,
            protocol_fee_recipient_token_account,
            base_token_program,
            quote_token_program,
            system_program,
            associated_token_program,
            event_authority,
            program,
            coin_creator_vault_ata,
            coin_creator_vault_authority,
            global_volume_accumulator,
            user_volume_accumulator,
            fee_config,
            fee_program,
        })
    }
}
