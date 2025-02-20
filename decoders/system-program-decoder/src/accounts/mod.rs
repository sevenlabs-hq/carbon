use {
    crate::SystemProgramDecoder,
    carbon_core::account::{AccountDecoder, DecodedAccount},
    solana_sdk::account::ReadableAccount,
};
pub enum SystemProgramAccount {
    Account(Vec<u8>),
}

impl AccountDecoder<'_> for SystemProgramDecoder {
    type AccountType = SystemProgramAccount;

    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner() != &solana_sdk::system_program::id() {
            return None;
        }

        Some(DecodedAccount {
            data: SystemProgramAccount::Account(account.data.clone()),
            lamports: account.lamports,
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        })
    }
}
