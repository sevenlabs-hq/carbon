use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xadff94067a638c16")]
pub struct LockRaydiumLiquidity {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LockRaydiumLiquidityInstructionAccounts {
    pub lock_program: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub fee_nft_owner: solana_pubkey::Pubkey,
    pub fee_nft_mint: solana_pubkey::Pubkey,
    pub fee_nft_account: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub locked_liquidity: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub liquidity_owner_lp: solana_pubkey::Pubkey,
    pub locked_lp_vault: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub metadata_account: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LockRaydiumLiquidity {
    type ArrangedAccounts = LockRaydiumLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lock_program, vault_authority, authority, fee_nft_owner, fee_nft_mint, fee_nft_account, pool_state, locked_liquidity, lp_mint, liquidity_owner_lp, locked_lp_vault, token_0_vault, token_1_vault, operator, config, bonding_curve, metadata_account, rent, system_program, token_program, associated_token_program, metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LockRaydiumLiquidityInstructionAccounts {
            lock_program: lock_program.pubkey,
            vault_authority: vault_authority.pubkey,
            authority: authority.pubkey,
            fee_nft_owner: fee_nft_owner.pubkey,
            fee_nft_mint: fee_nft_mint.pubkey,
            fee_nft_account: fee_nft_account.pubkey,
            pool_state: pool_state.pubkey,
            locked_liquidity: locked_liquidity.pubkey,
            lp_mint: lp_mint.pubkey,
            liquidity_owner_lp: liquidity_owner_lp.pubkey,
            locked_lp_vault: locked_lp_vault.pubkey,
            token_0_vault: token_0_vault.pubkey,
            token_1_vault: token_1_vault.pubkey,
            operator: operator.pubkey,
            config: config.pubkey,
            bonding_curve: bonding_curve.pubkey,
            metadata_account: metadata_account.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}
