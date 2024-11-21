use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe8f2c5fdf08f8134")]
pub struct CreateTokenLedger {}

pub struct CreateTokenLedgerInstructionAccounts {
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateTokenLedger {
    type ArrangedAccounts = CreateTokenLedgerInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_ledger = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateTokenLedgerInstructionAccounts {
            token_ledger: token_ledger.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
