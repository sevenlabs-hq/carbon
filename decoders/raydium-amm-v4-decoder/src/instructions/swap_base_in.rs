use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09")]
pub struct SwapBaseIn {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
pub struct SwapBaseInInstructionAccounts {
    pub spl_token_id: solana_sdk::pubkey::Pubkey,
    pub amm_id: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub serum_program_id: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub user_source_token_account: solana_sdk::pubkey::Pubkey,
    pub user_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub user_source_owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseIn{
    type ArrangedAccounts = SwapBaseInInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let spl_token_id = accounts.get(0)?;
        let amm_id = accounts.get(1)?;
        let amm_authority = accounts.get(2)?;
        let amm_open_orders = accounts.get(3)?;
        let amm_target_orders = accounts.get(4)?;
        let pool_coin_token_account = accounts.get(5)?;
        let pool_pc_token_account = accounts.get(6)?;
        let serum_program_id = accounts.get(7)?;
        let serum_market = accounts.get(8)?;
        let serum_bids = accounts.get(9)?;
        let serum_asks = accounts.get(10)?;
        let serum_event_queue = accounts.get(11)?;
        let serum_coin_vault_account = accounts.get(12)?;
        let serum_pc_vault_account = accounts.get(13)?;
        let serum_vault_signer = accounts.get(14)?;
        let user_source_token_account = accounts.get(15)?;
        let user_destination_token_account = accounts.get(16)?;
        let user_source_owner = accounts.get(17)?;

        Some(SwapBaseInInstructionAccounts {
            spl_token_id: spl_token_id.pubkey,
            amm_id: amm_id.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            serum_program_id: serum_program_id.pubkey,
            serum_market: serum_market.pubkey,
            serum_bids: serum_bids.pubkey,
            serum_asks: serum_asks.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
            serum_coin_vault_account: serum_coin_vault_account.pubkey,
            serum_pc_vault_account: serum_pc_vault_account.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            user_source_token_account: user_source_token_account.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            user_source_owner: user_source_owner.pubkey,
        })
    }
}
