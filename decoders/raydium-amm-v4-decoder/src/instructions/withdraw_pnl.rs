use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07")]
pub struct WithdrawPnl {}

pub struct WithdrawPnlInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub coin_pnl_token_account: solana_sdk::pubkey::Pubkey,
    pub pc_pnl_token_account: solana_sdk::pubkey::Pubkey,
    pub pnl_owner_account: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub referrer_pc_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawPnl {
    type ArrangedAccounts = WithdrawPnlInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let amm = accounts.get(1)?;
        let amm_config = accounts.get(2)?;
        let amm_authority = accounts.get(3)?;
        let amm_open_orders = accounts.get(4)?;
        let pool_coin_token_account = accounts.get(5)?;
        let pool_pc_token_account = accounts.get(6)?;
        let coin_pnl_token_account = accounts.get(7)?;
        let pc_pnl_token_account = accounts.get(8)?;
        let pnl_owner_account = accounts.get(9)?;
        let amm_target_orders = accounts.get(10)?;
        let serum_program = accounts.get(11)?;
        let serum_market = accounts.get(12)?;
        let serum_event_queue = accounts.get(13)?;
        let serum_coin_vault_account = accounts.get(14)?;
        let serum_pc_vault_account = accounts.get(15)?;
        let serum_vault_signer = accounts.get(16)?;
        let referrer_pc_account = accounts.get(17)?;

        Some(WithdrawPnlInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_config: amm_config.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            coin_pnl_token_account: coin_pnl_token_account.pubkey,
            pc_pnl_token_account: pc_pnl_token_account.pubkey,
            pnl_owner_account: pnl_owner_account.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
            serum_coin_vault_account: serum_coin_vault_account.pubkey,
            serum_pc_vault_account: serum_pc_vault_account.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            referrer_pc_account: referrer_pc_account.pubkey,
        })
    }
}
