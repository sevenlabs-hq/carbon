use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x64196302d9ef7cad")]
pub struct DeletePositionBundle {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeletePositionBundleInstructionAccounts {
    pub position_bundle: solana_pubkey::Pubkey,
    pub position_bundle_mint: solana_pubkey::Pubkey,
    pub position_bundle_token_account: solana_pubkey::Pubkey,
    pub position_bundle_owner: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeletePositionBundle {
    type ArrangedAccounts = DeletePositionBundleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let position_bundle = next_account(&mut iter)?;
        let position_bundle_mint = next_account(&mut iter)?;
        let position_bundle_token_account = next_account(&mut iter)?;
        let position_bundle_owner = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(DeletePositionBundleInstructionAccounts {
            position_bundle,
            position_bundle_mint,
            position_bundle_token_account,
            position_bundle_owner,
            receiver,
            token_program,
        })
    }
}
