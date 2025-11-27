use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3ec6d6c1d59f6cd2")]
pub struct Claim {
    pub id: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimInstructionAccounts {
    pub wallet: solana_pubkey::Pubkey,
    pub program_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Claim {
    type ArrangedAccounts = ClaimInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let wallet = next_account(&mut iter)?;
        let program_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(ClaimInstructionAccounts {
            wallet,
            program_authority,
            system_program,
        })
    }
}
