use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3a7fbc3e4f52c460")]
pub struct DecreaseLiquidityV2 {
    pub liquidity: u128,
    pub amount0_min: u64,
    pub amount1_min: u64,
}

#[derive(Debug, PartialEq)]
pub struct DecreaseLiquidityV2InstructionAccounts {
    pub nft_owner: solana_pubkey::Pubkey,
    pub nft_account: solana_pubkey::Pubkey,
    pub personal_position: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub protocol_position: solana_pubkey::Pubkey,
    pub token_vault0: solana_pubkey::Pubkey,
    pub token_vault1: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
    pub recipient_token_account0: solana_pubkey::Pubkey,
    pub recipient_token_account1: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub vault0_mint: solana_pubkey::Pubkey,
    pub vault1_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreaseLiquidityV2 {
    type ArrangedAccounts = DecreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nft_owner, nft_account, personal_position, pool_state, protocol_position, token_vault0, token_vault1, tick_array_lower, tick_array_upper, recipient_token_account0, recipient_token_account1, token_program, token_program2022, memo_program, vault0_mint, vault1_mint, _remaining @ ..] =
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
            token_vault0: token_vault0.pubkey,
            token_vault1: token_vault1.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            recipient_token_account0: recipient_token_account0.pubkey,
            recipient_token_account1: recipient_token_account1.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            memo_program: memo_program.pubkey,
            vault0_mint: vault0_mint.pubkey,
            vault1_mint: vault1_mint.pubkey,
        })
    }
}
