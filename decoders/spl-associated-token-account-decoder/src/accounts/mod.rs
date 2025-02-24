use super::SplAssociatedTokenAccountDecoder;
use carbon_core::account::AccountDecoder;

pub enum SplAssociatedTokenAccountAccount {}

impl<'a> AccountDecoder<'a> for SplAssociatedTokenAccountDecoder {
    type AccountType = SplAssociatedTokenAccountAccount;
    fn decode_account(
        &self,
        _account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
