use {
    super::SystemProgramDecoder,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod nonce;

pub enum SystemAccount {
    Nonce(nonce::Nonce),
}

impl AccountDecoder<'_> for SystemProgramDecoder {
    type AccountType = SystemAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&solana_program::system_program::id()) {
            return None;
        }

        if let Some(decoded_account) = nonce::Nonce::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SystemAccount::Nonce(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
