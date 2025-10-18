use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8888fcddc2427e59")]
pub struct CollectProtocolFee {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolFeeInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub vault_0_mint: solana_pubkey::Pubkey,
    pub vault_1_mint: solana_pubkey::Pubkey,
    pub recipient_token_0_account: solana_pubkey::Pubkey,
    pub recipient_token_1_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectProtocolFee {
    type ArrangedAccounts = CollectProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let vault_0_mint = next_account(&mut iter)?;
        let vault_1_mint = next_account(&mut iter)?;
        let recipient_token_0_account = next_account(&mut iter)?;
        let recipient_token_1_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;

        Some(CollectProtocolFeeInstructionAccounts {
            owner,
            authority,
            pool_state,
            amm_config,
            token_0_vault,
            token_1_vault,
            vault_0_mint,
            vault_1_mint,
            recipient_token_0_account,
            recipient_token_1_account,
            token_program,
            token_program_2022,
        })
    }
}
