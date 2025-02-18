use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::MoonshotDecoder;
pub mod config_account;
pub mod curve_account;

pub enum MoonshotAccount {
    ConfigAccount(config_account::ConfigAccount),
    CurveAccount(curve_account::CurveAccount),
}

impl<'a> AccountDecoder<'a> for MoonshotDecoder {
    type AccountType = MoonshotAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            config_account::ConfigAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MoonshotAccount::ConfigAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            curve_account::CurveAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MoonshotAccount::CurveAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
