use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2ae50ae7bd3ec1ae")]
pub struct MigrateFunds {}

pub struct MigrateFundsInstructionAccounts {
    pub backend_authority: solana_sdk::pubkey::Pubkey,
    pub migration_authority: solana_sdk::pubkey::Pubkey,
    pub curve_account: solana_sdk::pubkey::Pubkey,
    pub curve_token_account: solana_sdk::pubkey::Pubkey,
    pub migration_authority_token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub dex_fee_account: solana_sdk::pubkey::Pubkey,
    pub helio_fee_account: solana_sdk::pubkey::Pubkey,
    pub config_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateFunds {
    type ArrangedAccounts = MigrateFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [backend_authority, migration_authority, curve_account, curve_token_account, migration_authority_token_account, mint, dex_fee_account, helio_fee_account, config_account, system_program, token_program, associated_token_program] =
            accounts
        else {
            return None;
        };

        Some(MigrateFundsInstructionAccounts {
            backend_authority: backend_authority.pubkey,
            migration_authority: migration_authority.pubkey,
            curve_account: curve_account.pubkey,
            curve_token_account: curve_token_account.pubkey,
            migration_authority_token_account: migration_authority_token_account.pubkey,
            mint: mint.pubkey,
            dex_fee_account: dex_fee_account.pubkey,
            helio_fee_account: helio_fee_account.pubkey,
            config_account: config_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
