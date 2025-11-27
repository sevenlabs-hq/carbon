use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b2ef09b50045666")]
pub struct InitializeWhitelistInsuranceAccount {
    pub nonce: u8,
}

pub struct InitializeWhitelistInsuranceAccountInstructionAccounts {
    pub whitelist_insurance_account: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeWhitelistInsuranceAccount {
    type ArrangedAccounts = InitializeWhitelistInsuranceAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whitelist_insurance_account, admin, user, system_program, state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeWhitelistInsuranceAccountInstructionAccounts {
            whitelist_insurance_account: whitelist_insurance_account.pubkey,
            admin: admin.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            state: state.pubkey,
        })
    }
}
