use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd42f5f5c726683fa")]
pub struct OpenPositionWithTokenExtensions {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub with_token_metadata_extension: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct OpenPositionWithTokenExtensionsInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub token2022_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub metadata_update_auth: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPositionWithTokenExtensions {
    type ArrangedAccounts = OpenPositionWithTokenExtensionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_mint = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let metadata_update_auth = next_account(&mut iter)?;

        Some(OpenPositionWithTokenExtensionsInstructionAccounts {
            funder,
            owner,
            position,
            position_mint,
            position_token_account,
            whirlpool,
            token2022_program,
            system_program,
            associated_token_program,
            metadata_update_auth,
        })
    }
}
