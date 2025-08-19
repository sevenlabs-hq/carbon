use {
    super::WeightedSwapDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod pool;
pub mod vault;

pub enum WeightedSwapAccount {
    Pool(pool::Pool),
    Vault(vault::Vault),
}

impl AccountDecoder<'_> for WeightedSwapDecoder {
    type AccountType = WeightedSwapAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WeightedSwapAccount::Pool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = vault::Vault::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WeightedSwapAccount::Vault(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
