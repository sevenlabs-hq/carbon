use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14c6caedebf3b742")]
pub struct WithdrawLeftover {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WithdrawLeftoverInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub virtual_pool: solana_pubkey::Pubkey,
    pub token_base_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub leftover_receiver: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawLeftover {
    type ArrangedAccounts = WithdrawLeftoverInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let virtual_pool = next_account(&mut iter)?;
        let token_base_account = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let leftover_receiver = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(WithdrawLeftoverInstructionAccounts {
            pool_authority,
            config,
            virtual_pool,
            token_base_account,
            base_vault,
            base_mint,
            leftover_receiver,
            token_base_program,
            event_authority,
            program,
        })
    }
}
