use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1416567bc61cdb84")]
pub struct CollectCreatorFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectCreatorFeeInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub vault_0_mint: solana_pubkey::Pubkey,
    pub vault_1_mint: solana_pubkey::Pubkey,
    pub creator_token_0: solana_pubkey::Pubkey,
    pub creator_token_1: solana_pubkey::Pubkey,
    pub token_0_program: solana_pubkey::Pubkey,
    pub token_1_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectCreatorFee {
    type ArrangedAccounts = CollectCreatorFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let creator = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let vault_0_mint = next_account(&mut iter)?;
        let vault_1_mint = next_account(&mut iter)?;
        let creator_token_0 = next_account(&mut iter)?;
        let creator_token_1 = next_account(&mut iter)?;
        let token_0_program = next_account(&mut iter)?;
        let token_1_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CollectCreatorFeeInstructionAccounts {
            creator,
            authority,
            pool_state,
            amm_config,
            token_0_vault,
            token_1_vault,
            vault_0_mint,
            vault_1_mint,
            creator_token_0,
            creator_token_1,
            token_0_program,
            token_1_program,
            associated_token_program,
            system_program,
        })
    }
}
