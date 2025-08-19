use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9118acc2db7d03be")]
pub struct InitializeCustomizablePermissionlessConstantProductPool {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub params: CustomizableParams,
}

pub struct InitializeCustomizablePermissionlessConstantProductPoolInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub a_vault: solana_pubkey::Pubkey,
    pub b_vault: solana_pubkey::Pubkey,
    pub a_token_vault: solana_pubkey::Pubkey,
    pub b_token_vault: solana_pubkey::Pubkey,
    pub a_vault_lp_mint: solana_pubkey::Pubkey,
    pub b_vault_lp_mint: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub b_vault_lp: solana_pubkey::Pubkey,
    pub payer_token_a: solana_pubkey::Pubkey,
    pub payer_token_b: solana_pubkey::Pubkey,
    pub payer_pool_lp: solana_pubkey::Pubkey,
    pub protocol_token_a_fee: solana_pubkey::Pubkey,
    pub protocol_token_b_fee: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub mint_metadata: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub vault_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts
    for InitializeCustomizablePermissionlessConstantProductPool
{
    type ArrangedAccounts =
        InitializeCustomizablePermissionlessConstantProductPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, lp_mint, token_a_mint, token_b_mint, a_vault, b_vault, a_token_vault, b_token_vault, a_vault_lp_mint, b_vault_lp_mint, a_vault_lp, b_vault_lp, payer_token_a, payer_token_b, payer_pool_lp, protocol_token_a_fee, protocol_token_b_fee, payer, rent, mint_metadata, metadata_program, vault_program, token_program, associated_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            InitializeCustomizablePermissionlessConstantProductPoolInstructionAccounts {
                pool: pool.pubkey,
                lp_mint: lp_mint.pubkey,
                token_a_mint: token_a_mint.pubkey,
                token_b_mint: token_b_mint.pubkey,
                a_vault: a_vault.pubkey,
                b_vault: b_vault.pubkey,
                a_token_vault: a_token_vault.pubkey,
                b_token_vault: b_token_vault.pubkey,
                a_vault_lp_mint: a_vault_lp_mint.pubkey,
                b_vault_lp_mint: b_vault_lp_mint.pubkey,
                a_vault_lp: a_vault_lp.pubkey,
                b_vault_lp: b_vault_lp.pubkey,
                payer_token_a: payer_token_a.pubkey,
                payer_token_b: payer_token_b.pubkey,
                payer_pool_lp: payer_pool_lp.pubkey,
                protocol_token_a_fee: protocol_token_a_fee.pubkey,
                protocol_token_b_fee: protocol_token_b_fee.pubkey,
                payer: payer.pubkey,
                rent: rent.pubkey,
                mint_metadata: mint_metadata.pubkey,
                metadata_program: metadata_program.pubkey,
                vault_program: vault_program.pubkey,
                token_program: token_program.pubkey,
                associated_token_program: associated_token_program.pubkey,
                system_program: system_program.pubkey,
            },
        )
    }
}
