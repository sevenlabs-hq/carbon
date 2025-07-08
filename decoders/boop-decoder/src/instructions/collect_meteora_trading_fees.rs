use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf95f7e5b51a253fa")]
pub struct CollectMeteoraTradingFees {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectMeteoraTradingFeesInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub protocol_fee_recipient: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub token_a_account: solana_pubkey::Pubkey,
    pub token_b_account: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub cp_amm_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectMeteoraTradingFees {
    type ArrangedAccounts = CollectMeteoraTradingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [operator, protocol_fee_recipient, config, pool_authority, pool, position, token_a_account, token_b_account, token_a_vault, token_b_vault, token_a_mint, token_b_mint, position_nft_account, vault_authority, token_program, associated_token_program, event_authority, cp_amm_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectMeteoraTradingFeesInstructionAccounts {
            operator: operator.pubkey,
            protocol_fee_recipient: protocol_fee_recipient.pubkey,
            config: config.pubkey,
            pool_authority: pool_authority.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            token_a_account: token_a_account.pubkey,
            token_b_account: token_b_account.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            vault_authority: vault_authority.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            cp_amm_program: cp_amm_program.pubkey,
        })
    }
}
