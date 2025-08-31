use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa559dd34aaf9226f")]
pub struct ClaimStandardCreatorTradingFees {
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimStandardCreatorTradingFeesInstructionAccounts {
    pub token_a_program: solana_pubkey::Pubkey,
    pub token_b_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub liquidity_pool_state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub user_token_a_vault: solana_pubkey::Pubkey,
    pub user_token_b_vault: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
    pub instruction_sysvar_account_info: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimStandardCreatorTradingFees {
    type ArrangedAccounts = ClaimStandardCreatorTradingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_a_program, token_b_program, associated_token_program, system_program, liquidity_pool_state, user, token_a_mint, token_b_mint, user_token_a_vault, user_token_b_vault, token_a_vault, token_b_vault, protocol_config, instruction_sysvar_account_info, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimStandardCreatorTradingFeesInstructionAccounts {
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            liquidity_pool_state: liquidity_pool_state.pubkey,
            user: user.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            user_token_a_vault: user_token_a_vault.pubkey,
            user_token_b_vault: user_token_b_vault.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            protocol_config: protocol_config.pubkey,
            instruction_sysvar_account_info: instruction_sysvar_account_info.pubkey,
        })
    }
}
