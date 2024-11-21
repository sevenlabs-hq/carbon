use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct Deposit {
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

pub struct DepositInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub lp_mint_address: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub user_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub user_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub user_lp_token_account: solana_sdk::pubkey::Pubkey,
    pub user_owner: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let amm = accounts.get(1)?;
        let amm_authority = accounts.get(2)?;
        let amm_open_orders = accounts.get(3)?;
        let amm_target_orders = accounts.get(4)?;
        let lp_mint_address = accounts.get(5)?;
        let pool_coin_token_account = accounts.get(6)?;
        let pool_pc_token_account = accounts.get(7)?;
        let serum_market = accounts.get(8)?;
        let user_coin_token_account = accounts.get(9)?;
        let user_pc_token_account = accounts.get(10)?;
        let user_lp_token_account = accounts.get(11)?;
        let user_owner = accounts.get(12)?;
        let serum_event_queue = accounts.get(13)?;

        Some(DepositInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            lp_mint_address: lp_mint_address.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            serum_market: serum_market.pubkey,
            user_coin_token_account: user_coin_token_account.pubkey,
            user_pc_token_account: user_pc_token_account.pubkey,
            user_lp_token_account: user_lp_token_account.pubkey,
            user_owner: user_owner.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
        })
    }
}
