use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct Transfer {
    pub onwer: solana_sdk::pubkey::Pubkey,
}

pub struct TransferAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub recipient_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let funding_account = accounts.get(0)?;
        let recipient_account = accounts.get(1)?;

        Some(TransferAccounts {
            funding_account: funding_account.pubkey,
            recipient_account: recipient_account.pubkey,
        })
    }
}
