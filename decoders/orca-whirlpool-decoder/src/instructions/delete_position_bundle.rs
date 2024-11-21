use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x64196302d9ef7cad")]
pub struct DeletePositionBundle {}

pub struct DeletePositionBundleInstructionAccounts {
    pub position_bundle: solana_sdk::pubkey::Pubkey,
    pub position_bundle_mint: solana_sdk::pubkey::Pubkey,
    pub position_bundle_token_account: solana_sdk::pubkey::Pubkey,
    pub position_bundle_owner: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeletePositionBundle {
    type ArrangedAccounts = DeletePositionBundleInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let position_bundle = accounts.get(0)?;
        let position_bundle_mint = accounts.get(1)?;
        let position_bundle_token_account = accounts.get(2)?;
        let position_bundle_owner = accounts.get(3)?;
        let receiver = accounts.get(4)?;
        let token_program = accounts.get(5)?;

        Some(DeletePositionBundleInstructionAccounts {
            position_bundle: position_bundle.pubkey,
            position_bundle_mint: position_bundle_mint.pubkey,
            position_bundle_token_account: position_bundle_token_account.pubkey,
            position_bundle_owner: position_bundle_owner.pubkey,
            receiver: receiver.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
