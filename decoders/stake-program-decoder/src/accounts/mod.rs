use carbon_core::account::AccountDecoder;

use super::StakeProgramDecoder;

pub enum StakeProgramAccount {}

impl AccountDecoder<'_> for StakeProgramDecoder {
    type AccountType = StakeProgramAccount;
    fn decode_account(
        &self,
        _account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
