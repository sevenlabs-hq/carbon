use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xff5c572dc6acec02")]
pub struct PreInitialize {
    pub nonce: u8,
}

pub struct PreInitializeInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub pool_withdraw_queue: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub lp_mint_address: solana_sdk::pubkey::Pubkey,
    pub coin_mint_address: solana_sdk::pubkey::Pubkey,
    pub pc_mint_address: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_temp_lp_token_account: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub user_wallet: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PreInitialize {
    type ArrangedAccounts = PreInitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, system_program, rent, amm_target_orders, pool_withdraw_queue, amm_authority, lp_mint_address, coin_mint_address, pc_mint_address, pool_coin_token_account, pool_pc_token_account, pool_temp_lp_token_account, serum_market, user_wallet] =
            accounts
        else {
            return None;
        };

        Some(PreInitializeInstructionAccounts {
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_withdraw_queue: pool_withdraw_queue.pubkey,
            amm_authority: amm_authority.pubkey,
            lp_mint_address: lp_mint_address.pubkey,
            coin_mint_address: coin_mint_address.pubkey,
            pc_mint_address: pc_mint_address.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            pool_temp_lp_token_account: pool_temp_lp_token_account.pubkey,
            serum_market: serum_market.pubkey,
            user_wallet: user_wallet.pubkey,
        })
    }
}
