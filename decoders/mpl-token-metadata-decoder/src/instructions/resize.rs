use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x38")]
pub struct Resize {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ResizeInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub authority: Option<solana_pubkey::Pubkey>,
    pub token: Option<solana_pubkey::Pubkey>,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Resize {
    type ArrangedAccounts = ResizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let edition = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let authority = next_account(&mut iter);
        let token = next_account(&mut iter);
        let system_program = next_account(&mut iter)?;

        Some(ResizeInstructionAccounts {
            metadata,
            edition,
            mint,
            payer,
            authority,
            token,
            system_program,
        })
    }
}
