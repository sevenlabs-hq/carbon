use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2aec48a2f2182754")]
pub struct SwapBaseIn {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

pub struct SwapBaseInInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: Option<solana_sdk::pubkey::Pubkey>,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub uer_source_token_account: solana_sdk::pubkey::Pubkey,
    pub uer_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub user_source_owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseIn {
    type ArrangedAccounts = SwapBaseInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        match accounts.len() {
            17 => {
                let [token_program, amm, amm_authority, amm_open_orders, pool_coin_token_account, pool_pc_token_account, serum_program, serum_market, serum_bids, serum_asks, serum_event_queue, serum_coin_vault_account, serum_pc_vault_account, serum_vault_signer, uer_source_token_account, uer_destination_token_account, user_source_owner] =
                    accounts
                else {
                    return None;
                };

                Some(SwapBaseInInstructionAccounts {
                    token_program: token_program.pubkey,
                    amm: amm.pubkey,
                    amm_authority: amm_authority.pubkey,
                    amm_open_orders: amm_open_orders.pubkey,
                    amm_target_orders: None,
                    pool_coin_token_account: pool_coin_token_account.pubkey,
                    pool_pc_token_account: pool_pc_token_account.pubkey,
                    serum_program: serum_program.pubkey,
                    serum_market: serum_market.pubkey,
                    serum_bids: serum_bids.pubkey,
                    serum_asks: serum_asks.pubkey,
                    serum_event_queue: serum_event_queue.pubkey,
                    serum_coin_vault_account: serum_coin_vault_account.pubkey,
                    serum_pc_vault_account: serum_pc_vault_account.pubkey,
                    serum_vault_signer: serum_vault_signer.pubkey,
                    uer_source_token_account: uer_source_token_account.pubkey,
                    uer_destination_token_account: uer_destination_token_account.pubkey,
                    user_source_owner: user_source_owner.pubkey,
                })
            }
            18 => {
                let [token_program, amm, amm_authority, amm_open_orders, amm_target_orders, pool_coin_token_account, pool_pc_token_account, serum_program, serum_market, serum_bids, serum_asks, serum_event_queue, serum_coin_vault_account, serum_pc_vault_account, serum_vault_signer, uer_source_token_account, uer_destination_token_account, user_source_owner] =
                    accounts
                else {
                    return None;
                };

                Some(SwapBaseInInstructionAccounts {
                    token_program: token_program.pubkey,
                    amm: amm.pubkey,
                    amm_authority: amm_authority.pubkey,
                    amm_open_orders: amm_open_orders.pubkey,
                    amm_target_orders: Some(amm_target_orders.pubkey),
                    pool_coin_token_account: pool_coin_token_account.pubkey,
                    pool_pc_token_account: pool_pc_token_account.pubkey,
                    serum_program: serum_program.pubkey,
                    serum_market: serum_market.pubkey,
                    serum_bids: serum_bids.pubkey,
                    serum_asks: serum_asks.pubkey,
                    serum_event_queue: serum_event_queue.pubkey,
                    serum_coin_vault_account: serum_coin_vault_account.pubkey,
                    serum_pc_vault_account: serum_pc_vault_account.pubkey,
                    serum_vault_signer: serum_vault_signer.pubkey,
                    uer_source_token_account: uer_source_token_account.pubkey,
                    uer_destination_token_account: uer_destination_token_account.pubkey,
                    user_source_owner: user_source_owner.pubkey,
                })
            }
            _ => None,
        }
    }
}
