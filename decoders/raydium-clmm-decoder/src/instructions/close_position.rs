use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7b86510031446262")]
pub struct ClosePosition {}

pub struct ClosePositionInstructionAccounts {
    pub nft_owner: solana_sdk::pubkey::Pubkey,
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub position_nft_account: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePosition {
    type ArrangedAccounts = ClosePositionInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nft_owner = accounts.get(0)?;
        let position_nft_mint = accounts.get(1)?;
        let position_nft_account = accounts.get(2)?;
        let personal_position = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let token_program = accounts.get(5)?;

        Some(ClosePositionInstructionAccounts {
            nft_owner: nft_owner.pubkey,
            position_nft_mint: position_nft_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            personal_position: personal_position.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
