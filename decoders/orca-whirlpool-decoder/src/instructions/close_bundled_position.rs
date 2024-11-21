use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2924d8f51b556743")]
pub struct CloseBundledPosition {
    pub bundle_index: u16,
}

pub struct CloseBundledPositionInstructionAccounts {
    pub bundled_position: solana_sdk::pubkey::Pubkey,
    pub position_bundle: solana_sdk::pubkey::Pubkey,
    pub position_bundle_token_account: solana_sdk::pubkey::Pubkey,
    pub position_bundle_authority: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseBundledPosition {
    type ArrangedAccounts = CloseBundledPositionInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let bundled_position = accounts.get(0)?;
        let position_bundle = accounts.get(1)?;
        let position_bundle_token_account = accounts.get(2)?;
        let position_bundle_authority = accounts.get(3)?;
        let receiver = accounts.get(4)?;

        Some(CloseBundledPositionInstructionAccounts {
            bundled_position: bundled_position.pubkey,
            position_bundle: position_bundle.pubkey,
            position_bundle_token_account: position_bundle_token_account.pubkey,
            position_bundle_authority: position_bundle_authority.pubkey,
            receiver: receiver.pubkey,
        })
    }
}
