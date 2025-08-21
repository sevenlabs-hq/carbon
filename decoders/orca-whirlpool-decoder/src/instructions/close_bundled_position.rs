use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2924d8f51b556743")]
pub struct CloseBundledPosition {
    pub bundle_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseBundledPositionInstructionAccounts {
    pub bundled_position: solana_pubkey::Pubkey,
    pub position_bundle: solana_pubkey::Pubkey,
    pub position_bundle_token_account: solana_pubkey::Pubkey,
    pub position_bundle_authority: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseBundledPosition {
    type ArrangedAccounts = CloseBundledPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let bundled_position = next_account(&mut iter)?;
        let position_bundle = next_account(&mut iter)?;
        let position_bundle_token_account = next_account(&mut iter)?;
        let position_bundle_authority = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;

        Some(CloseBundledPositionInstructionAccounts {
            bundled_position,
            position_bundle,
            position_bundle_token_account,
            position_bundle_authority,
            receiver,
        })
    }
}
