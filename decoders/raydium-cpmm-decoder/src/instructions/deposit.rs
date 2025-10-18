use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub owner_lp_token: solana_pubkey::Pubkey,
    pub token_0_account: solana_pubkey::Pubkey,
    pub token_1_account: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub vault_0_mint: solana_pubkey::Pubkey,
    pub vault_1_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let owner_lp_token = next_account(&mut iter)?;
        let token_0_account = next_account(&mut iter)?;
        let token_1_account = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;
        let vault_0_mint = next_account(&mut iter)?;
        let vault_1_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;

        Some(DepositInstructionAccounts {
            owner,
            authority,
            pool_state,
            owner_lp_token,
            token_0_account,
            token_1_account,
            token_0_vault,
            token_1_vault,
            token_program,
            token_program_2022,
            vault_0_mint,
            vault_1_mint,
            lp_mint,
        })
    }
}
