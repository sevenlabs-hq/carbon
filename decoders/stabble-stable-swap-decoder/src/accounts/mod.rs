use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::StableSwapDecoder;
pub mod pool;
pub mod strategy;
pub mod vault;

pub enum StableSwapAccount {
    Pool(pool::Pool),
    Strategy(strategy::Strategy),
    Vault(vault::Vault),
}

impl<'a> AccountDecoder<'a> for StableSwapDecoder {
    type AccountType = StableSwapAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: StableSwapAccount::Pool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = strategy::Strategy::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: StableSwapAccount::Strategy(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = vault::Vault::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: StableSwapAccount::Vault(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
