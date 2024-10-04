use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct InitializeMultisig {
    pub m: u8,
}

pub struct InitializeMultisigAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl ArrangeAccounts for InitializeMultisig {
    type ArrangedAccounts = InitializeMultisigAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let rent = accounts.get(1)?;

        Some(InitializeMultisigAccounts {
            account: *account,
            rent: *rent,
            remaining_accounts: accounts
                .get(2..)
                .unwrap_or_default()
                .to_vec()
                .into_iter()
                .map(|pubkey| solana_sdk::instruction::AccountMeta {
                    pubkey,
                    is_signer: true,
                    is_writable: false,
                })
                .collect::<Vec<solana_sdk::instruction::AccountMeta>>(),
        })
    }
}
