use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6951552a04afef0")]
pub struct CollectDust {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectDustInstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub token_admin: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectDust {
    type ArrangedAccounts = CollectDustInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let vault = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let token_admin = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CollectDustInstructionAccounts {
            vault,
            token_vault,
            token_admin,
            admin,
            token_program,
        })
    }
}
