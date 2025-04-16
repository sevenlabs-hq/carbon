use carbon_core::{borsh, CarbonDeserialize};

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
        let [creator, beneficiary, pool_state, vesting_record, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateVestingAccountInstructionAccounts {
            creator: creator.pubkey,
            beneficiary: beneficiary.pubkey,
            pool_state: pool_state.pubkey,
            vesting_record: vesting_record.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
