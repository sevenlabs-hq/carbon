use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17")]
pub struct AmountToUiAmount {
    pub amount: u64,
}

pub struct AmountToUiAmountAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for AmountToUiAmount {
    type ArrangedAccounts = AmountToUiAmountAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(AmountToUiAmountAccounts { mint: mint.pubkey })
    }
}
