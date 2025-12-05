use super::SplAssociatedTokenAccountDecoder;
use carbon_core::account::{AccountDecoder, AccountMetadata};

pub enum SplAssociatedTokenAccountAccount {}

impl<'a> AccountDecoder<'a> for SplAssociatedTokenAccountDecoder {
    type AccountType = SplAssociatedTokenAccountAccount;
    fn decode_account(
        &self,
        _account: &'a solana_account::Account,
        _metadata: Option<&'a AccountMetadata>,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
