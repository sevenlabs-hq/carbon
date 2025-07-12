use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x27")]
pub struct CloseEscrowAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseEscrowAccountInstructionAccounts {
    pub escrow: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseEscrowAccount {
    type ArrangedAccounts = CloseEscrowAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let escrow = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_account = next_account(&mut iter)?;
        let edition = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;

        Some(CloseEscrowAccountInstructionAccounts {
            escrow,
            metadata,
            mint,
            token_account,
            edition,
            payer,
            system_program,
            sysvar_instructions,
        })
    }
}
