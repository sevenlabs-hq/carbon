use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xee9965a9f3832401")]
pub struct SetOperator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetOperatorInstructionAccounts {
    pub vault: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetOperator {
    type ArrangedAccounts = SetOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let vault = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;

        Some(SetOperatorInstructionAccounts {
            vault,
            operator,
            admin,
        })
    }
}
