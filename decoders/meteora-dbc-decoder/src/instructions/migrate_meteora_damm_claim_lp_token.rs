use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

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
        let mut iter = accounts.iter();
        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let source_token = next_account(&mut iter)?;
        let destination_token = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let sender = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(MigrateMeteoraDammClaimLpTokenInstructionAccounts {
            virtual_pool,
            migration_metadata,
            pool_authority,
            lp_mint,
            source_token,
            destination_token,
            owner,
            sender,
            token_program,
        })
    }
}
