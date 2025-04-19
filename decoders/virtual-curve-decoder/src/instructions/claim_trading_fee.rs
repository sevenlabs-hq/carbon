use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08ec5931987db151")]
pub struct ClaimTradingFee {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimTradingFeeInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub token_a_account: solana_pubkey::Pubkey,
    pub token_b_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimTradingFee {
    type ArrangedAccounts = ClaimTradingFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool_authority, config, pool, token_a_account, token_b_account, base_vault, quote_vault, base_mint, quote_mint, fee_claimer, token_base_program, token_quote_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimTradingFeeInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            config: config.pubkey,
            pool: pool.pubkey,
            token_a_account: token_a_account.pubkey,
            token_b_account: token_b_account.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            fee_claimer: fee_claimer.pubkey,
            token_base_program: token_base_program.pubkey,
            token_quote_program: token_quote_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
