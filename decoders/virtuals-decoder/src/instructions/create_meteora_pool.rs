use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6fe2125e1b029e8")]
pub struct CreateMeteoraPool {}

pub struct CreateMeteoraPoolInstructionAccounts {
    pub vpool: solana_pubkey::Pubkey,
    pub meteora_deployer: solana_pubkey::Pubkey,
    pub meteora_deployer_virtuals_ata: solana_pubkey::Pubkey,
    pub meteora_deployer_token_ata: solana_pubkey::Pubkey,
    pub vpool_virtuals_ata: solana_pubkey::Pubkey,
    pub vpool_token_ata: solana_pubkey::Pubkey,
    pub lock_escrow: solana_pubkey::Pubkey,
    pub escrow_vault: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub virtuals_mint: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub virtuals_vault: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub virtuals_token_vault: solana_pubkey::Pubkey,
    pub token_token_vault: solana_pubkey::Pubkey,
    pub virtuals_vault_lp_mint: solana_pubkey::Pubkey,
    pub token_vault_lp_mint: solana_pubkey::Pubkey,
    pub virtuals_vault_lp: solana_pubkey::Pubkey,
    pub token_vault_lp: solana_pubkey::Pubkey,
    pub pool_virtuals_ata: solana_pubkey::Pubkey,
    pub pool_token_ata: solana_pubkey::Pubkey,
    pub meteora_deployer_pool_lp: solana_pubkey::Pubkey,
    pub protocol_virtuals_fee: solana_pubkey::Pubkey,
    pub protocol_token_fee: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_metadata: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub mint_metadata: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub vault_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub dynamic_amm_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMeteoraPool {
    type ArrangedAccounts = CreateMeteoraPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [vpool, meteora_deployer, meteora_deployer_virtuals_ata, meteora_deployer_token_ata, vpool_virtuals_ata, vpool_token_ata, lock_escrow, escrow_vault, pool, config, lp_mint, virtuals_mint, token_mint, virtuals_vault, token_vault, virtuals_token_vault, token_token_vault, virtuals_vault_lp_mint, token_vault_lp_mint, virtuals_vault_lp, token_vault_lp, pool_virtuals_ata, pool_token_ata, meteora_deployer_pool_lp, protocol_virtuals_fee, protocol_token_fee, payer, token_metadata, rent, mint_metadata, metadata_program, vault_program, token_program, associated_token_program, system_program, dynamic_amm_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateMeteoraPoolInstructionAccounts {
            vpool: vpool.pubkey,
            meteora_deployer: meteora_deployer.pubkey,
            meteora_deployer_virtuals_ata: meteora_deployer_virtuals_ata.pubkey,
            meteora_deployer_token_ata: meteora_deployer_token_ata.pubkey,
            vpool_virtuals_ata: vpool_virtuals_ata.pubkey,
            vpool_token_ata: vpool_token_ata.pubkey,
            lock_escrow: lock_escrow.pubkey,
            escrow_vault: escrow_vault.pubkey,
            pool: pool.pubkey,
            config: config.pubkey,
            lp_mint: lp_mint.pubkey,
            virtuals_mint: virtuals_mint.pubkey,
            token_mint: token_mint.pubkey,
            virtuals_vault: virtuals_vault.pubkey,
            token_vault: token_vault.pubkey,
            virtuals_token_vault: virtuals_token_vault.pubkey,
            token_token_vault: token_token_vault.pubkey,
            virtuals_vault_lp_mint: virtuals_vault_lp_mint.pubkey,
            token_vault_lp_mint: token_vault_lp_mint.pubkey,
            virtuals_vault_lp: virtuals_vault_lp.pubkey,
            token_vault_lp: token_vault_lp.pubkey,
            pool_virtuals_ata: pool_virtuals_ata.pubkey,
            pool_token_ata: pool_token_ata.pubkey,
            meteora_deployer_pool_lp: meteora_deployer_pool_lp.pubkey,
            protocol_virtuals_fee: protocol_virtuals_fee.pubkey,
            protocol_token_fee: protocol_token_fee.pubkey,
            payer: payer.pubkey,
            token_metadata: token_metadata.pubkey,
            rent: rent.pubkey,
            mint_metadata: mint_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            dynamic_amm_program: dynamic_amm_program.pubkey,
        })
    }
}
