use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::JupiterDcaDecoder;
pub mod dca;

pub enum JupiterDcaAccount {
    Dca(dca::Dca),
}

impl<'a> AccountDecoder<'a> for JupiterDcaDecoder {
    type AccountType = JupiterDcaAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = dca::Dca::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterDcaAccount::Dca(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
