use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdd50b02599bca044")]
pub struct CreateTokenMetadata {
    pub params: CreateTokenMetadataParams,
}

pub struct CreateTokenMetadataInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub transfer_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub lp_token_mint: solana_pubkey::Pubkey,
    pub token_metadata_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateTokenMetadata {
    type ArrangedAccounts = CreateTokenMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, perpetuals, pool, transfer_authority, metadata, lp_token_mint, token_metadata_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateTokenMetadataInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            transfer_authority: transfer_authority.pubkey,
            metadata: metadata.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
