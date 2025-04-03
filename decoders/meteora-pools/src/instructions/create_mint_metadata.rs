use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0d46a829fa64945a")]
pub struct CreateMintMetadata {}

pub struct CreateMintMetadataInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub mint_metadata: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMintMetadata {
    type ArrangedAccounts = CreateMintMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, lp_mint, a_vault_lp, mint_metadata, metadata_program, system_program, payer, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateMintMetadataInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            mint_metadata: mint_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            system_program: system_program.pubkey,
            payer: payer.pubkey,
        })
    }
}
