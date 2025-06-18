use carbon_core::account::AccountDecoder;

use super::RaydiumStableSwapAmmDecoder;

pub enum RaydiumStableSwapAmmAccount {}

impl AccountDecoder<'_> for RaydiumStableSwapAmmDecoder {
    type AccountType = RaydiumStableSwapAmmAccount;
    fn decode_account(
        &self,
        _account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
