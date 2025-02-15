use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::MeteoraDlmmDecoder;
pub mod bin_array;
pub mod bin_array_bitmap_extension;
pub mod lb_pair;
pub mod oracle;
pub mod position;
pub mod position_v2;
pub mod preset_parameter;

pub enum MeteoraDlmmAccount {
    BinArrayBitmapExtension(bin_array_bitmap_extension::BinArrayBitmapExtension),
    BinArray(bin_array::BinArray),
    LbPair(lb_pair::LbPair),
    Oracle(oracle::Oracle),
    Position(position::Position),
    PositionV2(position_v2::PositionV2),
    PresetParameter(preset_parameter::PresetParameter),
}

impl<'a> AccountDecoder<'a> for MeteoraDlmmDecoder {
    type AccountType = MeteoraDlmmAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            bin_array_bitmap_extension::BinArrayBitmapExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArrayBitmapExtension(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = bin_array::BinArray::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArray(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lb_pair::LbPair::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::LbPair(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = oracle::Oracle::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::Oracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position_v2::PositionV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PositionV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            preset_parameter::PresetParameter::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PresetParameter(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
