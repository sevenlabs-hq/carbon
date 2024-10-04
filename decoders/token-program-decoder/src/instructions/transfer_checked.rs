use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct TransferChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct TransferCheckedAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub _remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl ArrangeAccounts for TransferChecked {
    type ArrangedAccounts = TransferCheckedAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let destination = accounts.get(1)?;
        let authority = accounts.get(2)?;

        Some(TransferCheckedAccounts {
            source: *source,
            destination: *destination,
            authority: *authority,
            _remaining_accounts: accounts
                .get(3..)
                .unwrap_or_default()
                .to_vec()
                .into_iter()
                .map(|pubkey| solana_sdk::instruction::AccountMeta {
                    pubkey,
                    is_signer: false,
                    is_writable: false,
                })
                .collect::<Vec<solana_sdk::instruction::AccountMeta>>(),
        })
    }
}
