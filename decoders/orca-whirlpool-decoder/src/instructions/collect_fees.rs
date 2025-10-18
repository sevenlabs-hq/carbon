use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa498cf631eba13b6")]
pub struct CollectFees {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectFeesInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub token_owner_account_a: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_owner_account_b: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFees {
    type ArrangedAccounts = CollectFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let position_authority = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let token_owner_account_a = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_owner_account_b = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CollectFeesInstructionAccounts {
            whirlpool,
            position_authority,
            position,
            position_token_account,
            token_owner_account_a,
            token_vault_a,
            token_owner_account_b,
            token_vault_b,
            token_program,
        })
    }
}
