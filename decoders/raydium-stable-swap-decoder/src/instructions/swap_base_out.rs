use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b")]
pub struct SwapBaseOut {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapBaseOutInstructionAccounts {
    pub spl_token_program: solana_pubkey::Pubkey,
    pub amm_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub pool_token_coin: solana_pubkey::Pubkey,
    pub pool_token_pc: solana_pubkey::Pubkey,
    pub serum_dex_program_id: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub event_q: solana_pubkey::Pubkey,
    pub coin_vault: solana_pubkey::Pubkey,
    pub pc_vault: solana_pubkey::Pubkey,
    pub vault_signer: solana_pubkey::Pubkey,
    pub user_source_token: solana_pubkey::Pubkey,
    pub user_destination_token: solana_pubkey::Pubkey,
    pub user_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseOut {
    type ArrangedAccounts = SwapBaseOutInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [spl_token_program, amm_account, authority, amm_open_orders, amm_target_orders, pool_token_coin, pool_token_pc, serum_dex_program_id, serum_market, bids, asks, event_q, coin_vault, pc_vault, vault_signer, user_source_token, user_destination_token, user_owner, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapBaseOutInstructionAccounts {
            spl_token_program: spl_token_program.pubkey,
            amm_account: amm_account.pubkey,
            authority: authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_token_coin: pool_token_coin.pubkey,
            pool_token_pc: pool_token_pc.pubkey,
            serum_dex_program_id: serum_dex_program_id.pubkey,
            serum_market: serum_market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_q: event_q.pubkey,
            coin_vault: coin_vault.pubkey,
            pc_vault: pc_vault.pubkey,
            vault_signer: vault_signer.pubkey,
            user_source_token: user_source_token.pubkey,
            user_destination_token: user_destination_token.pubkey,
            user_owner: user_owner.pubkey,
        })
    }
}
