use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x55a372798ba72925")]
pub struct InitializeInsuranceDepositAccount {
    pub nonce: u8,
}

pub struct InitializeInsuranceDepositAccountInstructionAccounts {
    pub insurance_deposit_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub whitelist_insurance_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeInsuranceDepositAccount {
    type ArrangedAccounts = InitializeInsuranceDepositAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [insurance_deposit_account, authority, payer, system_program, whitelist_insurance_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInsuranceDepositAccountInstructionAccounts {
            insurance_deposit_account: insurance_deposit_account.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            whitelist_insurance_account: whitelist_insurance_account.pubkey,
        })
    }
}
