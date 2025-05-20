use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xea66c2cb96483ee5")]
pub struct ExtendAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ExtendAccountInstructionAccounts {
    pub account: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExtendAccount {
    type ArrangedAccounts = ExtendAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, user, system_program, event_authority, program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(ExtendAccountInstructionAccounts {
            account: account.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
