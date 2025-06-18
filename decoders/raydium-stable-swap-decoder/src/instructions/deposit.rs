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

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositInstructionAccounts {
    pub spl_token_program: solana_pubkey::Pubkey,
    pub amm_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub amm_open_orders: solana_pubkey::Pubkey,
    pub amm_target_orders: solana_pubkey::Pubkey,
    pub pool_lp_mint: solana_pubkey::Pubkey,
    pub pool_token_coin: solana_pubkey::Pubkey,
    pub pool_token_pc: solana_pubkey::Pubkey,
    pub serum_market: solana_pubkey::Pubkey,
    pub user_coin_token: solana_pubkey::Pubkey,
    pub user_pc_token: solana_pubkey::Pubkey,
    pub user_lp_token: solana_pubkey::Pubkey,
    pub user_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [spl_token_program, amm_account, authority, amm_open_orders, amm_target_orders, pool_lp_mint, pool_token_coin, pool_token_pc, serum_market, user_coin_token, user_pc_token, user_lp_token, user_owner, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            spl_token_program: spl_token_program.pubkey,
            amm_account: amm_account.pubkey,
            authority: authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_lp_mint: pool_lp_mint.pubkey,
            pool_token_coin: pool_token_coin.pubkey,
            pool_token_pc: pool_token_pc.pubkey,
            serum_market: serum_market.pubkey,
            user_coin_token: user_coin_token.pubkey,
            user_pc_token: user_pc_token.pubkey,
            user_lp_token: user_lp_token.pubkey,
            user_owner: user_owner.pubkey,
        })
    }
}
