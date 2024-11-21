use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9717eabd5acd431")]
pub struct OpenBundledPosition {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

pub struct OpenBundledPositionInstructionAccounts {
    pub bundled_position: solana_sdk::pubkey::Pubkey,
    pub position_bundle: solana_sdk::pubkey::Pubkey,
    pub position_bundle_token_account: solana_sdk::pubkey::Pubkey,
    pub position_bundle_authority: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenBundledPosition {
    type ArrangedAccounts = OpenBundledPositionInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let bundled_position = accounts.get(0)?;
        let position_bundle = accounts.get(1)?;
        let position_bundle_token_account = accounts.get(2)?;
        let position_bundle_authority = accounts.get(3)?;
        let whirlpool = accounts.get(4)?;
        let funder = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;

        Some(OpenBundledPositionInstructionAccounts {
            bundled_position: bundled_position.pubkey,
            position_bundle: position_bundle.pubkey,
            position_bundle_token_account: position_bundle_token_account.pubkey,
            position_bundle_authority: position_bundle_authority.pubkey,
            whirlpool: whirlpool.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
