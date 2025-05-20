use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub user_base_token_account: solana_pubkey::Pubkey,
    pub user_quote_token_account: solana_pubkey::Pubkey,
    pub user_pool_token_account: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, global_config, user, base_mint, quote_mint, lp_mint, user_base_token_account, user_quote_token_account, user_pool_token_account, pool_base_token_account, pool_quote_token_account, token_program, token_2022_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            pool: pool.pubkey,
            global_config: global_config.pubkey,
            user: user.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            lp_mint: lp_mint.pubkey,
            user_base_token_account: user_base_token_account.pubkey,
            user_quote_token_account: user_quote_token_account.pubkey,
            user_pool_token_account: user_pool_token_account.pubkey,
            pool_base_token_account: pool_base_token_account.pubkey,
            pool_quote_token_account: pool_quote_token_account.pubkey,
            token_program: token_program.pubkey,
            token_2022_program: token_2022_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
