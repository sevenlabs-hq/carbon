use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x15")]
pub struct RevokeUseAuthority {}

pub struct RevokeUseAuthorityInstructionAccounts {
    pub use_authority_record: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub owner_token_account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RevokeUseAuthority {
    type ArrangedAccounts = RevokeUseAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [use_authority_record, owner, user, owner_token_account, mint, metadata, token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
