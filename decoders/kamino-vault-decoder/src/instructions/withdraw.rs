use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub shares_amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub withdraw_from_available: solana_sdk::pubkey::Pubkey,
    pub withdraw_from_reserve_accounts: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [withdraw_from_available, withdraw_from_reserve_accounts, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            withdraw_from_available: withdraw_from_available.pubkey,
            withdraw_from_reserve_accounts: withdraw_from_reserve_accounts.pubkey,
        })
    }
}
