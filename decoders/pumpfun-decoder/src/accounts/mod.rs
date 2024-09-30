use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PumpfunDecoder;

pub mod bonding_curve;
pub mod global;
pub mod last_withdraw;

pub enum PumpFunAccount {
    Global(global::Global),
    BondingCurve(bonding_curve::BondingCurve),
    LastWithdraw(last_withdraw::LastWithdraw),
}

impl AccountDecoder for PumpfunDecoder {
    type AccountType = PumpFunAccount;

    fn decode_account(
        &self,
        account: solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = global::Global::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpFunAccount::Global(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }
        if let Some(decoded_account) =
            bonding_curve::BondingCurve::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpFunAccount::BondingCurve(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }
        if let Some(decoded_account) =
            last_withdraw::LastWithdraw::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpFunAccount::LastWithdraw(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }
        None
    }
}
