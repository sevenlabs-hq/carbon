use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x33e685a4017f83ad")]
pub struct Sell {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SellInstructionAccounts {
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
}

impl carbon_core::deserialize::ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, user, global_config, base_mint, quote_mint, user_base_token_account, user_quote_token_account, pool_base_token_account, pool_quote_token_account, protocol_fee_recipient, protocol_fee_recipient_token_account, base_token_program, quote_token_program, system_program, associated_token_program, event_authority, program, coin_creator_vault_ata, coin_creator_vault_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SellInstructionAccounts {
            pool: pool.pubkey,
            user: user.pubkey,
            global_config: global_config.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            user_base_token_account: user_base_token_account.pubkey,
            user_quote_token_account: user_quote_token_account.pubkey,
            pool_base_token_account: pool_base_token_account.pubkey,
            pool_quote_token_account: pool_quote_token_account.pubkey,
            protocol_fee_recipient: protocol_fee_recipient.pubkey,
            protocol_fee_recipient_token_account: protocol_fee_recipient_token_account.pubkey,
            base_token_program: base_token_program.pubkey,
            quote_token_program: quote_token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            coin_creator_vault_ata: coin_creator_vault_ata.pubkey,
            coin_creator_vault_authority: coin_creator_vault_authority.pubkey,
        })
    }
}
