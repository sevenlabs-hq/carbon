use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8b85021e5b917f9a")]
pub struct MigrateMeteoraDammClaimLpToken {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateMeteoraDammClaimLpTokenInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub source_token: solana_pubkey::Pubkey,
    pub destination_token: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateMeteoraDammClaimLpToken {
    type ArrangedAccounts = MigrateMeteoraDammClaimLpTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, migration_metadata, pool_authority, lp_mint, source_token, destination_token, owner, sender, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateMeteoraDammClaimLpTokenInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            migration_metadata: migration_metadata.pubkey,
            pool_authority: pool_authority.pubkey,
            lp_mint: lp_mint.pubkey,
            source_token: source_token.pubkey,
            destination_token: destination_token.pubkey,
            owner: owner.pubkey,
            sender: sender.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
