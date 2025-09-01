use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x50066f49aed34284")]
pub struct Withdraw2 {
    pub unmint_amount: u64,
    pub min_out_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct Withdraw2InstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub user_token: solana_pubkey::Pubkey,
    pub user_lp: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw2 {
    type ArrangedAccounts = Withdraw2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let vault = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let user_token = next_account(&mut iter)?;
        let user_lp = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(Withdraw2InstructionAccounts {
            vault,
            token_vault,
            lp_mint,
            user_token,
            user_lp,
            user,
            token_program,
        })
    }
}
