use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3121681ebd9d4f23")]
pub struct ClaimVestedToken {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimVestedTokenInstructionAccounts {
    pub beneficiary: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub vesting_record: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub user_base_token: solana_pubkey::Pubkey,
    pub base_token_mint: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimVestedToken {
    type ArrangedAccounts = ClaimVestedTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let beneficiary = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let vesting_record = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let user_base_token = next_account(&mut iter)?;
        let base_token_mint = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;

        Some(ClaimVestedTokenInstructionAccounts {
            beneficiary,
            authority,
            pool_state,
            vesting_record,
            base_vault,
            user_base_token,
            base_token_mint,
            base_token_program,
            system_program,
            associated_token_program,
        })
    }
}
