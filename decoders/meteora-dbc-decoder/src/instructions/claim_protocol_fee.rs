use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa5e4853063f9ff21")]
pub struct ClaimProtocolFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimProtocolFeeInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_base_account: solana_pubkey::Pubkey,
    pub token_quote_account: solana_pubkey::Pubkey,
    pub claim_fee_operator: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimProtocolFee {
    type ArrangedAccounts = ClaimProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_base_account = next_account(&mut iter)?;
        let token_quote_account = next_account(&mut iter)?;
        let claim_fee_operator = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(ClaimProtocolFeeInstructionAccounts {
            pool_authority,
            config,
            pool,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            token_base_account,
            token_quote_account,
            claim_fee_operator,
            operator,
            token_base_program,
            token_quote_program,
            event_authority,
            program,
        })
    }
}
