use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11")]
pub struct SyncNative {}

pub struct SyncNativeInstructionAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SyncNative {
    type ArrangedAccounts = SyncNativeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SyncNativeInstructionAccounts {
            account: account.pubkey,
        })
    }
}
