use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0d")]
pub struct AdminCancelOrders {
    pub limit: u16,
}

pub struct AdminCancelOrdersInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub amm_owner_account: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub serum_event_q: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminCancelOrders {
    type ArrangedAccounts = AdminCancelOrdersInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let amm = accounts.get(1)?;
        let amm_authority = accounts.get(2)?;
        let amm_open_orders = accounts.get(3)?;
        let amm_target_orders = accounts.get(4)?;
        let pool_coin_token_account = accounts.get(5)?;
        let pool_pc_token_account = accounts.get(6)?;
        let amm_owner_account = accounts.get(7)?;
        let amm_config = accounts.get(8)?;
        let serum_program = accounts.get(9)?;
        let serum_market = accounts.get(10)?;
        let serum_coin_vault_account = accounts.get(11)?;
        let serum_pc_vault_account = accounts.get(12)?;
        let serum_vault_signer = accounts.get(13)?;
        let serum_event_q = accounts.get(14)?;
        let serum_bids = accounts.get(15)?;
        let serum_asks = accounts.get(16)?;

        Some(AdminCancelOrdersInstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            amm_owner_account: amm_owner_account.pubkey,
            amm_config: amm_config.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_coin_vault_account: serum_coin_vault_account.pubkey,
            serum_pc_vault_account: serum_pc_vault_account.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            serum_event_q: serum_event_q.pubkey,
            serum_bids: serum_bids.pubkey,
            serum_asks: serum_asks.pubkey,
        })
    }
}
