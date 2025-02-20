use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14")]
pub struct ApproveUseAuthority {
    pub approve_use_authority_args: ApproveUseAuthorityArgs,
}

pub struct ApproveUseAuthorityInstructionAccounts {
    pub use_authority_record: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub owner_token_account: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub burner: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ApproveUseAuthority {
    type ArrangedAccounts = ApproveUseAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [use_authority_record, owner, payer, user, owner_token_account, metadata, mint, burner, token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ApproveUseAuthorityInstructionAccounts {
            use_authority_record: use_authority_record.pubkey,
            owner: owner.pubkey,
            payer: payer.pubkey,
            user: user.pubkey,
            owner_token_account: owner_token_account.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            burner: burner.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
