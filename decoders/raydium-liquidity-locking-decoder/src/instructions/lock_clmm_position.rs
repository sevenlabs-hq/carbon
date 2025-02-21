use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbc25b38352965449")]
pub struct LockClmmPosition {
    pub with_metadata: bool,
}

pub struct LockClmmPositionInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub position_nft_owner: solana_sdk::pubkey::Pubkey,
    pub fee_nft_owner: solana_sdk::pubkey::Pubkey,
    pub position_nft_account: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub locked_nft_account: solana_sdk::pubkey::Pubkey,
    pub locked_position: solana_sdk::pubkey::Pubkey,
    pub fee_nft_mint: solana_sdk::pubkey::Pubkey,
    pub fee_nft_account: solana_sdk::pubkey::Pubkey,
    pub metadata_account: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub fee_nft_token_program: solana_sdk::pubkey::Pubkey,
    pub locked_nft_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LockClmmPosition {
    type ArrangedAccounts = LockClmmPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, payer, position_nft_owner, fee_nft_owner, position_nft_account, personal_position, position_nft_mint, locked_nft_account, locked_position, fee_nft_mint, fee_nft_account, metadata_account, metadata_program, associated_token_program, rent, fee_nft_token_program, locked_nft_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LockClmmPositionInstructionAccounts {
            authority: authority.pubkey,
            payer: payer.pubkey,
            position_nft_owner: position_nft_owner.pubkey,
            fee_nft_owner: fee_nft_owner.pubkey,
            position_nft_account: position_nft_account.pubkey,
            personal_position: personal_position.pubkey,
            position_nft_mint: position_nft_mint.pubkey,
            locked_nft_account: locked_nft_account.pubkey,
            locked_position: locked_position.pubkey,
            fee_nft_mint: fee_nft_mint.pubkey,
            fee_nft_account: fee_nft_account.pubkey,
            metadata_account: metadata_account.pubkey,
            metadata_program: metadata_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
            fee_nft_token_program: fee_nft_token_program.pubkey,
            locked_nft_token_program: locked_nft_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
