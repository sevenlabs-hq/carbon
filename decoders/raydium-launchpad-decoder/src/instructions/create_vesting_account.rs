use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x81b2020dd9ace6da")]
pub struct CreateVestingAccount {
    pub share_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateVestingAccountInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub vesting_record: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateVestingAccount {
    type ArrangedAccounts = CreateVestingAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let creator = next_account(&mut iter)?;
        let beneficiary = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let vesting_record = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateVestingAccountInstructionAccounts {
            creator,
            beneficiary,
            pool_state,
            vesting_record,
            system_program,
        })
    }
}
