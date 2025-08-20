use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5fc8472208090ba6")]
pub struct SellExactOut {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SellExactOutInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub user_base_token: solana_pubkey::Pubkey,
    pub user_quote_token: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_token_mint: solana_pubkey::Pubkey,
    pub quote_token_mint: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellExactOut {
    type ArrangedAccounts = SellExactOutInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let user_base_token = next_account(&mut iter)?;
        let user_quote_token = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let base_token_mint = next_account(&mut iter)?;
        let quote_token_mint = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SellExactOutInstructionAccounts {
            payer,
            authority,
            global_config,
            platform_config,
            pool_state,
            user_base_token,
            user_quote_token,
            base_vault,
            quote_vault,
            base_token_mint,
            quote_token_mint,
            base_token_program,
            quote_token_program,
            event_authority,
            program,
        })
    }
}
