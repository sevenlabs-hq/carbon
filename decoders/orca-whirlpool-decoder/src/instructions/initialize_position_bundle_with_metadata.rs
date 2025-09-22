use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5d7c10b3f98373f5")]
pub struct InitializePositionBundleWithMetadata {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionBundleWithMetadataInstructionAccounts {
    pub position_bundle: solana_pubkey::Pubkey,
    pub position_bundle_mint: solana_pubkey::Pubkey,
    pub position_bundle_metadata: solana_pubkey::Pubkey,
    pub position_bundle_token_account: solana_pubkey::Pubkey,
    pub position_bundle_owner: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub metadata_update_auth: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePositionBundleWithMetadata {
    type ArrangedAccounts = InitializePositionBundleWithMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let position_bundle = next_account(&mut iter)?;
        let position_bundle_mint = next_account(&mut iter)?;
        let position_bundle_metadata = next_account(&mut iter)?;
        let position_bundle_token_account = next_account(&mut iter)?;
        let position_bundle_owner = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let metadata_update_auth = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;

        Some(InitializePositionBundleWithMetadataInstructionAccounts {
            position_bundle,
            position_bundle_mint,
            position_bundle_metadata,
            position_bundle_token_account,
            position_bundle_owner,
            funder,
            metadata_update_auth,
            token_program,
            system_program,
            rent,
            associated_token_program,
            metadata_program,
        })
    }
}
