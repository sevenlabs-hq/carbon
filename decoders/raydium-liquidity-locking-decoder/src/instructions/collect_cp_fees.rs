use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x081e33c7d1b8f785")]
pub struct CollectCpFees {
    pub fee_lp_amount: u64,
}

pub struct CollectCpFeesInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub fee_nft_owner: solana_sdk::pubkey::Pubkey,
    pub fee_nft_account: solana_sdk::pubkey::Pubkey,
    pub locked_liquidity: solana_sdk::pubkey::Pubkey,
    pub cp_swap_program: solana_sdk::pubkey::Pubkey,
    pub cp_authority: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub recipient_token0_account: solana_sdk::pubkey::Pubkey,
    pub recipient_token1_account: solana_sdk::pubkey::Pubkey,
    pub token0_vault: solana_sdk::pubkey::Pubkey,
    pub token1_vault: solana_sdk::pubkey::Pubkey,
    pub vault0_mint: solana_sdk::pubkey::Pubkey,
    pub vault1_mint: solana_sdk::pubkey::Pubkey,
    pub locked_lp_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectCpFees {
    type ArrangedAccounts = CollectCpFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, fee_nft_owner, fee_nft_account, locked_liquidity, cp_swap_program, cp_authority, pool_state, lp_mint, recipient_token0_account, recipient_token1_account, token0_vault, token1_vault, vault0_mint, vault1_mint, locked_lp_vault, token_program, token_program2022, memo_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectCpFeesInstructionAccounts {
            authority: authority.pubkey,
            fee_nft_owner: fee_nft_owner.pubkey,
            fee_nft_account: fee_nft_account.pubkey,
            locked_liquidity: locked_liquidity.pubkey,
            cp_swap_program: cp_swap_program.pubkey,
            cp_authority: cp_authority.pubkey,
            pool_state: pool_state.pubkey,
            lp_mint: lp_mint.pubkey,
            recipient_token0_account: recipient_token0_account.pubkey,
            recipient_token1_account: recipient_token1_account.pubkey,
            token0_vault: token0_vault.pubkey,
            token1_vault: token1_vault.pubkey,
            vault0_mint: vault0_mint.pubkey,
            vault1_mint: vault1_mint.pubkey,
            locked_lp_vault: locked_lp_vault.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
