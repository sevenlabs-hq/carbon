use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd89d1d4e26331f1a")]
pub struct LockCpLiquidity {
    pub lp_amount: u64,
    pub with_metadata: bool,
}

pub struct LockCpLiquidityInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub liquidity_owner: solana_sdk::pubkey::Pubkey,
    pub fee_nft_owner: solana_sdk::pubkey::Pubkey,
    pub fee_nft_mint: solana_sdk::pubkey::Pubkey,
    pub fee_nft_account: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub locked_liquidity: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub liquidity_owner_lp: solana_sdk::pubkey::Pubkey,
    pub locked_lp_vault: solana_sdk::pubkey::Pubkey,
    pub token0_vault: solana_sdk::pubkey::Pubkey,
    pub token1_vault: solana_sdk::pubkey::Pubkey,
    pub metadata_account: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LockCpLiquidity {
    type ArrangedAccounts = LockCpLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, payer, liquidity_owner, fee_nft_owner, fee_nft_mint, fee_nft_account, pool_state, locked_liquidity, lp_mint, liquidity_owner_lp, locked_lp_vault, token0_vault, token1_vault, metadata_account, rent, system_program, token_program, associated_token_program, metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LockCpLiquidityInstructionAccounts {
            authority: authority.pubkey,
            payer: payer.pubkey,
            liquidity_owner: liquidity_owner.pubkey,
            fee_nft_owner: fee_nft_owner.pubkey,
            fee_nft_mint: fee_nft_mint.pubkey,
            fee_nft_account: fee_nft_account.pubkey,
            pool_state: pool_state.pubkey,
            locked_liquidity: locked_liquidity.pubkey,
            lp_mint: lp_mint.pubkey,
            liquidity_owner_lp: liquidity_owner_lp.pubkey,
            locked_lp_vault: locked_lp_vault.pubkey,
            token0_vault: token0_vault.pubkey,
            token1_vault: token1_vault.pubkey,
            metadata_account: metadata_account.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}
