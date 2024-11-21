use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0a")]
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
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let system_program = accounts.get(1)?;
        let rent = accounts.get(2)?;
        let amm_target_orders = accounts.get(3)?;
        let pool_withdraw_queue = accounts.get(4)?;
        let amm_authority = accounts.get(5)?;
        let lp_mint_address = accounts.get(6)?;
        let coin_mint_address = accounts.get(7)?;
        let pc_mint_address = accounts.get(8)?;
        let pool_coin_token_account = accounts.get(9)?;
        let pool_pc_token_account = accounts.get(10)?;
        let pool_temp_lp_token_account = accounts.get(11)?;
        let serum_market = accounts.get(12)?;
        let user_wallet = accounts.get(13)?;

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
