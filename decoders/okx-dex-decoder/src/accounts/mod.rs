use carbon_core::account::{AccountDecoder, AccountMetadata};

use super::OkxDexDecoder;

pub enum OkxDexAccount {}

impl<'a> AccountDecoder<'a> for OkxDexDecoder {
    type AccountType = OkxDexAccount;
    fn decode_account(
        &self,
        _account: &'a solana_account::Account,
        _metadata: Option<&'a AccountMetadata>,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
