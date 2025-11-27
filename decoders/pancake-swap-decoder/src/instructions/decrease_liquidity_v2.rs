use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3a7fbc3e4f52c460")]
pub struct DecreaseLiquidityV2 {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityV2InstructionAccounts {
    pub nft_owner: solana_pubkey::Pubkey,
    pub nft_account: solana_pubkey::Pubkey,
    pub personal_position: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub protocol_position: solana_pubkey::Pubkey,
    pub token_vault_0: solana_pubkey::Pubkey,
    pub token_vault_1: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
    pub recipient_token_account_0: solana_pubkey::Pubkey,
    pub recipient_token_account_1: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub vault_0_mint: solana_pubkey::Pubkey,
    pub vault_1_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreaseLiquidityV2 {
    type ArrangedAccounts = DecreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nft_owner, nft_account, personal_position, pool_state, protocol_position, token_vault_0, token_vault_1, tick_array_lower, tick_array_upper, recipient_token_account_0, recipient_token_account_1, token_program, token_program_2022, memo_program, vault_0_mint, vault_1_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DecreaseLiquidityV2InstructionAccounts {
            nft_owner: nft_owner.pubkey,
            nft_account: nft_account.pubkey,
            personal_position: personal_position.pubkey,
            pool_state: pool_state.pubkey,
            protocol_position: protocol_position.pubkey,
            token_vault_0: token_vault_0.pubkey,
            token_vault_1: token_vault_1.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            recipient_token_account_0: recipient_token_account_0.pubkey,
            recipient_token_account_1: recipient_token_account_1.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
            memo_program: memo_program.pubkey,
            vault_0_mint: vault_0_mint.pubkey,
            vault_1_mint: vault_1_mint.pubkey,
        })
    }
}
