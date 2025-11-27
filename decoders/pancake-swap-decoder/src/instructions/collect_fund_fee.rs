use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa78a4e95dfc2067e")]
pub struct CollectFundFee {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectFundFeeInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub token_vault_0: solana_pubkey::Pubkey,
    pub token_vault_1: solana_pubkey::Pubkey,
    pub vault_0_mint: solana_pubkey::Pubkey,
    pub vault_1_mint: solana_pubkey::Pubkey,
    pub recipient_token_account_0: solana_pubkey::Pubkey,
    pub recipient_token_account_1: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFundFee {
    type ArrangedAccounts = CollectFundFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, pool_state, amm_config, token_vault_0, token_vault_1, vault_0_mint, vault_1_mint, recipient_token_account_0, recipient_token_account_1, token_program, token_program_2022, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectFundFeeInstructionAccounts {
            owner: owner.pubkey,
            pool_state: pool_state.pubkey,
            amm_config: amm_config.pubkey,
            token_vault_0: token_vault_0.pubkey,
            token_vault_1: token_vault_1.pubkey,
            vault_0_mint: vault_0_mint.pubkey,
            vault_1_mint: vault_1_mint.pubkey,
            recipient_token_account_0: recipient_token_account_0.pubkey,
            recipient_token_account_1: recipient_token_account_1.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
        })
    }
}
