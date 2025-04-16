use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf52c091fecf91df")]
pub struct MigrateToAmm {
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub market_vault_signer_nonce: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateToAmmInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub openbook_program: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub request_queue: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub market_vault_signer: solana_pubkey::Pubkey,
    pub market_base_vault: solana_pubkey::Pubkey,
    pub market_quote_vault: solana_pubkey::Pubkey,
    pub amm_program: solana_pubkey::Pubkey,
    pub amm_pool: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_lp_mint: solana_pubkey::Pubkey,
    pub amm_base_vault: solana_pubkey::Pubkey,
    pub amm_quote_vault: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub amm_create_fee_destination: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub pool_lp_token: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateToAmm {
    type ArrangedAccounts = MigrateToAmmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, base_mint, quote_mint, openbook_program, market, request_queue, event_queue, bids, asks, market_vault_signer, market_base_vault, market_quote_vault, amm_program, amm_pool, amm_authority, amm_open_orders, amm_lp_mint, amm_base_vault, amm_quote_vault, amm_target_orders, amm_config, amm_create_fee_destination, authority, pool_state, global_config, base_vault, quote_vault, pool_lp_token, spl_token_program, associated_token_program, system_program, rent_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateToAmmInstructionAccounts {
            payer: payer.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            openbook_program: openbook_program.pubkey,
            market: market.pubkey,
            request_queue: request_queue.pubkey,
            event_queue: event_queue.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            market_vault_signer: market_vault_signer.pubkey,
            market_base_vault: market_base_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            amm_program: amm_program.pubkey,
            amm_pool: amm_pool.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_lp_mint: amm_lp_mint.pubkey,
            amm_base_vault: amm_base_vault.pubkey,
            amm_quote_vault: amm_quote_vault.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            amm_config: amm_config.pubkey,
            amm_create_fee_destination: amm_create_fee_destination.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            global_config: global_config.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            pool_lp_token: pool_lp_token.pubkey,
            spl_token_program: spl_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent_program: rent_program.pubkey,
        })
    }
}
