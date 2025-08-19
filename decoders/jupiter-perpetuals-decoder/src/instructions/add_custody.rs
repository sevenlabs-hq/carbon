use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf7fe7e111a06d775")]
pub struct AddCustody {
    pub params: AddCustodyParams,
}

pub struct AddCustodyInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub transfer_authority: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_token_account: solana_pubkey::Pubkey,
    pub custody_token_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddCustody {
    type ArrangedAccounts = AddCustodyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, transfer_authority, perpetuals, pool, custody, custody_token_account, custody_token_mint, system_program, token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddCustodyInstructionAccounts {
            admin: admin.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_token_account: custody_token_account.pubkey,
            custody_token_mint: custody_token_mint.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
