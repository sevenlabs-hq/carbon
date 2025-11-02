use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::SystemProgramDecoder;

pub mod legacy;
pub mod nonce;

pub enum SystemAccount {
    Nonce(nonce::Nonce),
    Legacy(legacy::Legacy),
}

impl AccountDecoder<'_> for SystemProgramDecoder {
    type AccountType = SystemAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&solana_system_interface::program::id()) {
            return None;
        }

        if account.data.is_empty() {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SystemAccount::Legacy(legacy::Legacy),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
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
