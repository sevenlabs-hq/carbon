use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2924d8f51b556743")]
pub struct CloseBundledPosition {
    pub bundle_index: u16,
}

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
        let [bundled_position, position_bundle, position_bundle_token_account, position_bundle_authority, receiver, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseBundledPositionInstructionAccounts {
            bundled_position: bundled_position.pubkey,
            position_bundle: position_bundle.pubkey,
            position_bundle_token_account: position_bundle_token_account.pubkey,
            position_bundle_authority: position_bundle_authority.pubkey,
            receiver: receiver.pubkey,
        })
    }
}
