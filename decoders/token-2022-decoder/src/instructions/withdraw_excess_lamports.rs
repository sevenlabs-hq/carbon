use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x26")]
pub struct WithdrawExcessLamports {}

pub struct WithdrawExcessLamportsInstructionAccounts {
    pub source_account: solana_sdk::pubkey::Pubkey,
    pub destination_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawExcessLamports {
    type ArrangedAccounts = WithdrawExcessLamportsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source_account, destination_account, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(WithdrawExcessLamportsInstructionAccounts {
            source_account: source_account.pubkey,
            destination_account: destination_account.pubkey,
            authority: authority.pubkey,
        })
    }
}
