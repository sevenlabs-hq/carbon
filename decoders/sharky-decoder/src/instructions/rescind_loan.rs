use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4040a0d33324b19e")]
pub struct RescindLoan {}

pub struct RescindLoanInstructionAccounts {
    pub loan: solana_sdk::pubkey::Pubkey,
    pub lender_value_token_account: solana_sdk::pubkey::Pubkey,
    pub lender: solana_sdk::pubkey::Pubkey,
    pub value_mint: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RescindLoan {
    type ArrangedAccounts = RescindLoanInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let loan = accounts.get(0)?;
        let lender_value_token_account = accounts.get(1)?;
        let lender = accounts.get(2)?;
        let value_mint = accounts.get(3)?;
        let escrow = accounts.get(4)?;
        let escrow_token_account = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(RescindLoanInstructionAccounts {
            loan: loan.pubkey,
            lender_value_token_account: lender_value_token_account.pubkey,
            lender: lender.pubkey,
            value_mint: value_mint.pubkey,
            escrow: escrow.pubkey,
            escrow_token_account: escrow_token_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
