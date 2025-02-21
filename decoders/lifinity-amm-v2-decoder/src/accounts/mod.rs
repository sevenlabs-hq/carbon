use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::LifinityAmmV2Decoder;
pub mod amm;

pub enum LifinityAmmV2Account {
    Amm(amm::Amm),
}

impl<'a> AccountDecoder<'a> for LifinityAmmV2Decoder {
    type AccountType = LifinityAmmV2Account;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = amm::Amm::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LifinityAmmV2Account::Amm(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
