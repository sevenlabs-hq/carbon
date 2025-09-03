use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PumpFeesDecoder;
pub mod fee_config;

pub enum PumpFeesAccount {
    FeeConfig(fee_config::FeeConfig),
}

impl AccountDecoder<'_> for PumpFeesDecoder {
    type AccountType = PumpFeesAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = fee_config::FeeConfig::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpFeesAccount::FeeConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
