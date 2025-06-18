use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct Initialize {
    pub nonce: u8,
    pub open_time: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeInstructionAccounts {
    pub spl_token_program: solana_pubkey::Pubkey,
    pub amm_id: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub pool_lp_mint: solana_pubkey::Pubkey,
    pub coin_mint: solana_pubkey::Pubkey,
    pub pc_mint: solana_pubkey::Pubkey,
    pub pool_token_coin: solana_pubkey::Pubkey,
    pub pool_token_pc: solana_pubkey::Pubkey,
    pub withdraw_queue: solana_pubkey::Pubkey,
    pub token_dest_lp: solana_pubkey::Pubkey,
    pub token_temp_lp: solana_pubkey::Pubkey,
    pub serum_dex_program_id: solana_pubkey::Pubkey,
    pub serum_dex_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [spl_token_program, amm_id, amm_authority, amm_open_orders, pool_lp_mint, coin_mint, pc_mint, pool_token_coin, pool_token_pc, withdraw_queue, token_dest_lp, token_temp_lp, serum_dex_program_id, serum_dex_market, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            spl_token_program: spl_token_program.pubkey,
            amm_id: amm_id.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            pool_lp_mint: pool_lp_mint.pubkey,
            coin_mint: coin_mint.pubkey,
            pc_mint: pc_mint.pubkey,
            pool_token_coin: pool_token_coin.pubkey,
            pool_token_pc: pool_token_pc.pubkey,
            withdraw_queue: withdraw_queue.pubkey,
            token_dest_lp: token_dest_lp.pubkey,
            token_temp_lp: token_temp_lp.pubkey,
            serum_dex_program_id: serum_dex_program_id.pubkey,
            serum_dex_market: serum_dex_market.pubkey,
        })
    }
}
