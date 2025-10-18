use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod bonding_curve;
pub mod fee_config;
pub mod global;
pub mod global_volume_accumulator;
pub mod user_volume_accumulator;

pub enum PumpfunAccount {
    BondingCurve(bonding_curve::BondingCurve),
    FeeConfig(fee_config::FeeConfig),
    Global(global::Global),
    GlobalVolumeAccumulator(global_volume_accumulator::GlobalVolumeAccumulator),
    UserVolumeAccumulator(user_volume_accumulator::UserVolumeAccumulator),
}

impl AccountDecoder<'_> for PumpfunDecoder {
    type AccountType = PumpfunAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            bonding_curve::BondingCurve::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::BondingCurve(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = fee_config::FeeConfig::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::FeeConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = global::Global::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::Global(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            global_volume_accumulator::GlobalVolumeAccumulator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::GlobalVolumeAccumulator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            user_volume_accumulator::UserVolumeAccumulator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::UserVolumeAccumulator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
