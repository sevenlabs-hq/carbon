use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x407b7fe3c0eac614")]
pub struct AddStrategy {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddStrategyInstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub strategy: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddStrategy {
    type ArrangedAccounts = AddStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let vault = next_account(&mut iter)?;
        let strategy = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;

        Some(AddStrategyInstructionAccounts {
            vault,
            strategy,
            admin,
        })
    }
}
