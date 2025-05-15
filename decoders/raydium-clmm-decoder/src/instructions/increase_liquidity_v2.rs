use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x851d59df45eeb00a")]
pub struct IncreaseLiquidityV2 {
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
    pub base_flag: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub struct IncreaseLiquidityV2InstructionAccounts {
    pub nft_owner: solana_pubkey::Pubkey,
    pub nft_account: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub protocol_position: solana_pubkey::Pubkey,
    pub personal_position: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
    pub token_account0: solana_pubkey::Pubkey,
    pub token_account1: solana_pubkey::Pubkey,
    pub token_vault0: solana_pubkey::Pubkey,
    pub token_vault1: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program2022: solana_pubkey::Pubkey,
    pub vault0_mint: solana_pubkey::Pubkey,
    pub vault1_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreaseLiquidityV2 {
    type ArrangedAccounts = IncreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nft_owner, nft_account, pool_state, protocol_position, personal_position, tick_array_lower, tick_array_upper, token_account0, token_account1, token_vault0, token_vault1, token_program, token_program2022, vault0_mint, vault1_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(IncreaseLiquidityV2InstructionAccounts {
            nft_owner: nft_owner.pubkey,
            nft_account: nft_account.pubkey,
            pool_state: pool_state.pubkey,
            protocol_position: protocol_position.pubkey,
            personal_position: personal_position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            token_account0: token_account0.pubkey,
            token_account1: token_account1.pubkey,
            token_vault0: token_vault0.pubkey,
            token_vault1: token_vault1.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            vault0_mint: vault0_mint.pubkey,
            vault1_mint: vault1_mint.pubkey,
        })
    }
}
