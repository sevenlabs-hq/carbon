use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04")]
pub struct Withdraw {
    pub amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub lp_mint_address: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub user_lp_token_account: solana_sdk::pubkey::Pubkey,
    pub user_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub user_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub user_lp_owner: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub referrer_pc_wallet: Option<solana_sdk::pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        // inspired by https://github.com/raydium-io/raydium-amm/blob/master/program/src/processor.rs#L1882 just wrote differently
        const LOWER_BOUND_ACCOUNT_LEN: usize = 20;
        let input_accounts_len = accounts.len();

        if input_accounts_len != LOWER_BOUND_ACCOUNT_LEN
            && input_accounts_len != LOWER_BOUND_ACCOUNT_LEN + 1
            && input_accounts_len != LOWER_BOUND_ACCOUNT_LEN + 2
            && input_accounts_len != LOWER_BOUND_ACCOUNT_LEN + 3
        {
            return None;
        }
        let mut accounts = accounts.into_iter();

        let token_program = accounts.next()?;
        let amm = accounts.next()?;
        let amm_authority = accounts.next()?;
        let amm_open_orders = accounts.next()?;
        let amm_target_orders = accounts.next()?;
        let lp_mint_address = accounts.next()?;
        let pool_coin_token_account = accounts.next()?;
        let pool_pc_token_account = accounts.next()?;

        if input_accounts_len == LOWER_BOUND_ACCOUNT_LEN + 2
            || input_accounts_len == LOWER_BOUND_ACCOUNT_LEN + 3
        {
            let _ = accounts.next();
            let _ = accounts.next();
        }

        let serum_program = accounts.next()?;
        let serum_market = accounts.next()?;
        let serum_coin_vault_account = accounts.next()?;
        let serum_pc_vault_account = accounts.next()?;
        let serum_vault_signer = accounts.next()?;

        let user_lp_token_account = accounts.next()?;
        let user_coin_token_account = accounts.next()?;
        let user_pc_token_account = accounts.next()?;
        let user_lp_owner = accounts.next()?;

        let serum_event_queue = accounts.next()?;
        let serum_bids = accounts.next()?;
        let serum_asks = accounts.next()?;
        let referrer_pc_wallet = {
            if input_accounts_len == LOWER_BOUND_ACCOUNT_LEN + 1
                || input_accounts_len == LOWER_BOUND_ACCOUNT_LEN + 3
            {
                accounts.next()
            } else {
                None
            }
        };

        Some(WithdrawInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            lp_mint_address: lp_mint_address.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_coin_vault_account: serum_coin_vault_account.pubkey,
            serum_pc_vault_account: serum_pc_vault_account.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            user_lp_token_account: user_lp_token_account.pubkey,
            user_coin_token_account: user_coin_token_account.pubkey,
            user_pc_token_account: user_pc_token_account.pubkey,
            user_lp_owner: user_lp_owner.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
            serum_bids: serum_bids.pubkey,
            serum_asks: serum_asks.pubkey,
            referrer_pc_wallet: referrer_pc_wallet.map(|a| a.pubkey),
        })
    }
}
