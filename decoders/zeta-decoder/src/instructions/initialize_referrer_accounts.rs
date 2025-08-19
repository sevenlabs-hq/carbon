use {
    alloc::string::String,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x69e448ddda12b375")]
pub struct InitializeReferrerAccounts {
    pub referrer_id: String,
}

pub struct InitializeReferrerAccountsInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub referrer_id_account: solana_pubkey::Pubkey,
    pub referrer_pubkey_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReferrerAccounts {
    type ArrangedAccounts = InitializeReferrerAccountsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, referrer_id_account, referrer_pubkey_account, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeReferrerAccountsInstructionAccounts {
            authority: authority.pubkey,
            referrer_id_account: referrer_id_account.pubkey,
            referrer_pubkey_account: referrer_pubkey_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
