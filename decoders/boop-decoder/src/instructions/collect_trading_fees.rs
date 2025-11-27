use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbd26cdea514d1901")]
pub struct CollectTradingFees {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectTradingFeesInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub protocol_fee_recipient: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub lock_program: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub fee_nft_account: solana_pubkey::Pubkey,
    pub locked_liquidity: solana_pubkey::Pubkey,
    pub cpmm_program: solana_pubkey::Pubkey,
    pub cp_authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub recipient_token_0_account: solana_pubkey::Pubkey,
    pub recipient_token_1_account: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub vault_0_mint: solana_pubkey::Pubkey,
    pub vault_1_mint: solana_pubkey::Pubkey,
    pub locked_lp_vault: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectTradingFees {
    type ArrangedAccounts = CollectTradingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [operator, protocol_fee_recipient, config, lock_program, vault_authority, authority, fee_nft_account, locked_liquidity, cpmm_program, cp_authority, pool_state, lp_mint, recipient_token_0_account, recipient_token_1_account, token_0_vault, token_1_vault, vault_0_mint, vault_1_mint, locked_lp_vault, system_program, associated_token_program, token_program, token_program_2022, memo_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectTradingFeesInstructionAccounts {
            operator: operator.pubkey,
            protocol_fee_recipient: protocol_fee_recipient.pubkey,
            config: config.pubkey,
            lock_program: lock_program.pubkey,
            vault_authority: vault_authority.pubkey,
            authority: authority.pubkey,
            fee_nft_account: fee_nft_account.pubkey,
            locked_liquidity: locked_liquidity.pubkey,
            cpmm_program: cpmm_program.pubkey,
            cp_authority: cp_authority.pubkey,
            pool_state: pool_state.pubkey,
            lp_mint: lp_mint.pubkey,
            recipient_token_0_account: recipient_token_0_account.pubkey,
            recipient_token_1_account: recipient_token_1_account.pubkey,
            token_0_vault: token_0_vault.pubkey,
            token_1_vault: token_1_vault.pubkey,
            vault_0_mint: vault_0_mint.pubkey,
            vault_1_mint: vault_1_mint.pubkey,
            locked_lp_vault: locked_lp_vault.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
