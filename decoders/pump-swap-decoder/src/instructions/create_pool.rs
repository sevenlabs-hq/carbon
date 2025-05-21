use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe992d18ecf6840bc")]
pub struct CreatePool {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub coin_creator: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreatePoolInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub user_base_token_account: solana_pubkey::Pubkey,
    pub user_quote_token_account: solana_pubkey::Pubkey,
    pub user_pool_token_account: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePool {
    type ArrangedAccounts = CreatePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, global_config, creator, base_mint, quote_mint, lp_mint, user_base_token_account, user_quote_token_account, user_pool_token_account, pool_base_token_account, pool_quote_token_account, system_program, token_2022_program, base_token_program, quote_token_program, associated_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreatePoolInstructionAccounts {
            pool: pool.pubkey,
            global_config: global_config.pubkey,
            creator: creator.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            lp_mint: lp_mint.pubkey,
            user_base_token_account: user_base_token_account.pubkey,
            user_quote_token_account: user_quote_token_account.pubkey,
            user_pool_token_account: user_pool_token_account.pubkey,
            pool_base_token_account: pool_base_token_account.pubkey,
            pool_quote_token_account: pool_quote_token_account.pubkey,
            system_program: system_program.pubkey,
            token_2022_program: token_2022_program.pubkey,
            base_token_program: base_token_program.pubkey,
            quote_token_program: quote_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
