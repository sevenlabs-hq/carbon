use carbon_core::account::{AccountDecoder, AccountMetadata};

use super::RaydiumStableSwapAmmDecoder;

pub enum RaydiumStableSwapAmmAccount {}

impl<'a> AccountDecoder<'a> for RaydiumStableSwapAmmDecoder {
    type AccountType = RaydiumStableSwapAmmAccount;
    fn decode_account(
        &self,
        _account: &'a solana_account::Account,
        _metadata: Option<&'a AccountMetadata>,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
