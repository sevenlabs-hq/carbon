use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1048fac60ea2d413")]
pub struct CollectClmmFeesAndRewards {}

pub struct CollectClmmFeesAndRewardsInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub fee_nft_owner: solana_sdk::pubkey::Pubkey,
    pub fee_nft_account: solana_sdk::pubkey::Pubkey,
    pub locked_position: solana_sdk::pubkey::Pubkey,
    pub clmm_program: solana_sdk::pubkey::Pubkey,
    pub locked_nft_account: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_position: solana_sdk::pubkey::Pubkey,
    pub token0_vault: solana_sdk::pubkey::Pubkey,
    pub token1_vault: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub recipient_token0_account: solana_sdk::pubkey::Pubkey,
    pub recipient_token1_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub vault0_mint: solana_sdk::pubkey::Pubkey,
    pub vault1_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectClmmFeesAndRewards {
    type ArrangedAccounts = CollectClmmFeesAndRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, fee_nft_owner, fee_nft_account, locked_position, clmm_program, locked_nft_account, personal_position, pool_state, protocol_position, token0_vault, token1_vault, tick_array_lower, tick_array_upper, recipient_token0_account, recipient_token1_account, token_program, token_program2022, memo_program, vault0_mint, vault1_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectClmmFeesAndRewardsInstructionAccounts {
            authority: authority.pubkey,
            fee_nft_owner: fee_nft_owner.pubkey,
            fee_nft_account: fee_nft_account.pubkey,
            locked_position: locked_position.pubkey,
            clmm_program: clmm_program.pubkey,
            locked_nft_account: locked_nft_account.pubkey,
            personal_position: personal_position.pubkey,
            pool_state: pool_state.pubkey,
            protocol_position: protocol_position.pubkey,
            token0_vault: token0_vault.pubkey,
            token1_vault: token1_vault.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            recipient_token0_account: recipient_token0_account.pubkey,
            recipient_token1_account: recipient_token1_account.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            memo_program: memo_program.pubkey,
            vault0_mint: vault0_mint.pubkey,
            vault1_mint: vault1_mint.pubkey,
        })
    }
}
