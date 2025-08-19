use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x30335c7a51137029")]
pub struct TestInit {
    pub params: TestInitParams,
}

pub struct TestInitInstructionAccounts {
    pub upgrade_authority: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub transfer_authority: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TestInit {
    type ArrangedAccounts = TestInitInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [upgrade_authority, admin, transfer_authority, perpetuals, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TestInitInstructionAccounts {
            upgrade_authority: upgrade_authority.pubkey,
            admin: admin.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
