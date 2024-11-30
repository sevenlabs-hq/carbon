

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x15")]
pub struct RevokeUseAuthority{
}

pub struct RevokeUseAuthorityInstructionAccounts {
    pub use_authority_record: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub owner_token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RevokeUseAuthority {
    type ArrangedAccounts = RevokeUseAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let use_authority_record = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let user = accounts.get(2)?;
        let owner_token_account = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;

        Some(RevokeUseAuthorityInstructionAccounts {
            use_authority_record: use_authority_record.pubkey,
            owner: owner.pubkey,
            user: user.pubkey,
            owner_token_account: owner_token_account.pubkey,
            mint: mint.pubkey,
            metadata: metadata.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}