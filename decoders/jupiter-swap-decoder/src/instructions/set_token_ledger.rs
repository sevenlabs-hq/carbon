use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe455b9704e4f4d02")]
pub struct SetTokenLedger {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetTokenLedgerInstructionAccounts {
    pub token_ledger: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenLedger {
    type ArrangedAccounts = SetTokenLedgerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_ledger = next_account(&mut iter)?;
        let token_account = next_account(&mut iter)?;

        Some(SetTokenLedgerInstructionAccounts {
            token_ledger,
            token_account,
        })
    }
}
