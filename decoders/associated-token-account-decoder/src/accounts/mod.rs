use {super::SplAssociatedTokenAccountDecoder, carbon_core::account::AccountDecoder};

pub enum SplAssociatedTokenAccountAccount {}

impl AccountDecoder<'_> for SplAssociatedTokenAccountDecoder {
    type AccountType = SplAssociatedTokenAccountAccount;
    fn decode_account(
        &self,
        _account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
