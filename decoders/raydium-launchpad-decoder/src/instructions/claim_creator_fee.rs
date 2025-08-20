use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a618acb84ab8dfc")]
pub struct ClaimCreatorFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimCreatorFeeInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub fee_vault_authority: solana_pubkey::Pubkey,
    pub creator_fee_vault: solana_pubkey::Pubkey,
    pub recipient_token_account: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimCreatorFee {
    type ArrangedAccounts = ClaimCreatorFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let creator = next_account(&mut iter)?;
        let fee_vault_authority = next_account(&mut iter)?;
        let creator_fee_vault = next_account(&mut iter)?;
        let recipient_token_account = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;

        Some(ClaimCreatorFeeInstructionAccounts {
            creator,
            fee_vault_authority,
            creator_fee_vault,
            recipient_token_account,
            quote_mint,
            token_program,
            system_program,
            associated_token_program,
        })
    }
}
