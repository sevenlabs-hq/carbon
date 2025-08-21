use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa8ad4864c962265c")]
pub struct PartnerWithdrawSurplus {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PartnerWithdrawSurplusInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub virtual_pool: solana_pubkey::Pubkey,
    pub token_quote_account: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PartnerWithdrawSurplus {
    type ArrangedAccounts = PartnerWithdrawSurplusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let virtual_pool = next_account(&mut iter)?;
        let token_quote_account = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let fee_claimer = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(PartnerWithdrawSurplusInstructionAccounts {
            pool_authority,
            config,
            virtual_pool,
            token_quote_account,
            quote_vault,
            quote_mint,
            fee_claimer,
            token_quote_program,
            event_authority,
            program,
        })
    }
}
