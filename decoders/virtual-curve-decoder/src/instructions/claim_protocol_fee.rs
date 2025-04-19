use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa5e4853063f9ff21")]
pub struct ClaimProtocolFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimProtocolFeeInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_base_account: solana_pubkey::Pubkey,
    pub token_quote_account: solana_pubkey::Pubkey,
    pub claim_fee_operator: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimProtocolFee {
    type ArrangedAccounts = ClaimProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool_authority, config, pool, base_vault, quote_vault, base_mint, quote_mint, token_base_account, token_quote_account, claim_fee_operator, operator, token_base_program, token_quote_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimProtocolFeeInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            config: config.pubkey,
            pool: pool.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            token_base_account: token_base_account.pubkey,
            token_quote_account: token_quote_account.pubkey,
            claim_fee_operator: claim_fee_operator.pubkey,
            operator: operator.pubkey,
            token_base_program: token_base_program.pubkey,
            token_quote_program: token_quote_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
