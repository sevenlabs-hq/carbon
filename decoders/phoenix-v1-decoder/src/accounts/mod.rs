use super::PhoenixV1Decoder;
use carbon_core::account::AccountDecoder;

pub enum PhoenixV1Account {}

impl<'a> AccountDecoder<'a> for PhoenixV1Decoder {
    type AccountType = PhoenixV1Account;
    fn decode_account(
        &self,
        _account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
