use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct InitializeNonceAccount(solana_sdk::pubkey::Pubkey);

pub struct InitializeNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
    pub recent_blockhashes_sysvar: solana_sdk::pubkey::Pubkey,
    pub rent_sysvar: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeNonceAccount {
    type ArrangedAccounts = InitializeNonceAccountAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;
        let recent_blockhashes_sysvar = accounts.get(1)?;
        let rent_sysvar = accounts.get(2)?;

        Some(InitializeNonceAccountAccounts {
            nonce_account: nonce_account.pubkey,
            recent_blockhashes_sysvar: recent_blockhashes_sysvar.pubkey,
            rent_sysvar: rent_sysvar.pubkey,
        })
    }
}
