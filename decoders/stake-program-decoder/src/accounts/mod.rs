use {super::StakeProgramDecoder, carbon_core::account::AccountDecoder};

pub enum StakeProgramAccount {}

impl AccountDecoder<'_> for StakeProgramDecoder {
    type AccountType = StakeProgramAccount;
    fn decode_account(
        &self,
        _account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
