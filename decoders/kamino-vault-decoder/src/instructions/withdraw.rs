

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw{
    pub shares_amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub withdraw_from_available: solana_sdk::pubkey::Pubkey,
    pub withdraw_from_reserve_accounts: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let withdraw_from_available = accounts.get(0)?;
        let withdraw_from_reserve_accounts = accounts.get(1)?;

        Some(WithdrawInstructionAccounts {
            withdraw_from_available: withdraw_from_available.pubkey,
            withdraw_from_reserve_accounts: withdraw_from_reserve_accounts.pubkey,
        })
    }
}