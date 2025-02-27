use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::OkxDexDecoder;

pub enum OkxDexAccount {}

impl<'a> AccountDecoder<'a> for OkxDexDecoder {
    type AccountType = OkxDexAccount;
    fn decode_account(
        &self,
        _account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
