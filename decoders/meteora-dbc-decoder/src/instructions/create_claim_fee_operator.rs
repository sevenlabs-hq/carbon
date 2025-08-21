use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa93ecf6b3abba26d")]
pub struct CreateClaimFeeOperator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateClaimFeeOperatorInstructionAccounts {
    pub claim_fee_operator: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateClaimFeeOperator {
    type ArrangedAccounts = CreateClaimFeeOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let claim_fee_operator = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CreateClaimFeeOperatorInstructionAccounts {
            claim_fee_operator,
            operator,
            admin,
            system_program,
            event_authority,
            program,
        })
    }
}
