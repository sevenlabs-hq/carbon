use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe455b9704e4f4d02")]
pub struct SetTokenLedger {}

pub struct SetTokenLedgerInstructionAccounts {
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenLedger {
    type ArrangedAccounts = SetTokenLedgerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_ledger, token_account, token_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTokenLedgerInstructionAccounts {
            token_ledger: token_ledger.pubkey,
            token_account: token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
