use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x607ee12fb9d5323a")]
pub struct SplitTradingFees {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SplitTradingFeesInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub wsol: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub trading_fees_vault: solana_pubkey::Pubkey,
    pub fee_splitter_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub fee_splitter_config: solana_pubkey::Pubkey,
    pub fee_splitter_creator_vault: solana_pubkey::Pubkey,
    pub fee_splitter_vault_authority: solana_pubkey::Pubkey,
    pub fee_splitter_creator_vault_authority: solana_pubkey::Pubkey,
    pub fee_splitter_staking_mint: solana_pubkey::Pubkey,
    pub fee_splitter_wsol_vault: solana_pubkey::Pubkey,
    pub fee_splitter_creator_vault_authority_wsol_vault: solana_pubkey::Pubkey,
    pub fee_splitter_treasury_wsol_vault: solana_pubkey::Pubkey,
    pub fee_splitter_team_wsol_vault: solana_pubkey::Pubkey,
    pub fee_splitter_reward_pool: solana_pubkey::Pubkey,
    pub fee_splitter_reward_pool_staking_vault: solana_pubkey::Pubkey,
    pub fee_splitter_reward_pool_reward_vault: solana_pubkey::Pubkey,
    pub fee_splitter_reward_pool_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SplitTradingFees {
    type ArrangedAccounts = SplitTradingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [operator, mint, wsol, config, vault_authority, bonding_curve, trading_fees_vault, fee_splitter_program, system_program, token_program, associated_token_program, fee_splitter_config, fee_splitter_creator_vault, fee_splitter_vault_authority, fee_splitter_creator_vault_authority, fee_splitter_staking_mint, fee_splitter_wsol_vault, fee_splitter_creator_vault_authority_wsol_vault, fee_splitter_treasury_wsol_vault, fee_splitter_team_wsol_vault, fee_splitter_reward_pool, fee_splitter_reward_pool_staking_vault, fee_splitter_reward_pool_reward_vault, fee_splitter_reward_pool_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SplitTradingFeesInstructionAccounts {
            operator: operator.pubkey,
            mint: mint.pubkey,
            wsol: wsol.pubkey,
            config: config.pubkey,
            vault_authority: vault_authority.pubkey,
            bonding_curve: bonding_curve.pubkey,
            trading_fees_vault: trading_fees_vault.pubkey,
            fee_splitter_program: fee_splitter_program.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            fee_splitter_config: fee_splitter_config.pubkey,
            fee_splitter_creator_vault: fee_splitter_creator_vault.pubkey,
            fee_splitter_vault_authority: fee_splitter_vault_authority.pubkey,
            fee_splitter_creator_vault_authority: fee_splitter_creator_vault_authority.pubkey,
            fee_splitter_staking_mint: fee_splitter_staking_mint.pubkey,
            fee_splitter_wsol_vault: fee_splitter_wsol_vault.pubkey,
            fee_splitter_creator_vault_authority_wsol_vault:
                fee_splitter_creator_vault_authority_wsol_vault.pubkey,
            fee_splitter_treasury_wsol_vault: fee_splitter_treasury_wsol_vault.pubkey,
            fee_splitter_team_wsol_vault: fee_splitter_team_wsol_vault.pubkey,
            fee_splitter_reward_pool: fee_splitter_reward_pool.pubkey,
            fee_splitter_reward_pool_staking_vault: fee_splitter_reward_pool_staking_vault.pubkey,
            fee_splitter_reward_pool_reward_vault: fee_splitter_reward_pool_reward_vault.pubkey,
            fee_splitter_reward_pool_program: fee_splitter_reward_pool_program.pubkey,
        })
    }
}
