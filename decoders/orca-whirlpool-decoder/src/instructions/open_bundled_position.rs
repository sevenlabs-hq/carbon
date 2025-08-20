use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9717eabd5acd431")]
pub struct OpenBundledPosition {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct OpenBundledPositionInstructionAccounts {
    pub bundled_position: solana_pubkey::Pubkey,
    pub position_bundle: solana_pubkey::Pubkey,
    pub position_bundle_token_account: solana_pubkey::Pubkey,
    pub position_bundle_authority: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenBundledPosition {
    type ArrangedAccounts = OpenBundledPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let bundled_position = next_account(&mut iter)?;
        let position_bundle = next_account(&mut iter)?;
        let position_bundle_token_account = next_account(&mut iter)?;
        let position_bundle_authority = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(OpenBundledPositionInstructionAccounts {
            bundled_position,
            position_bundle,
            position_bundle_token_account,
            position_bundle_authority,
            whirlpool,
            funder,
            system_program,
            rent,
        })
    }
}
