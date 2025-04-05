use {
    super::MeteoraDlmmDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod bin_array;
pub mod bin_array_bitmap_extension;
pub mod lb_pair;
pub mod oracle;
pub mod position;
pub mod position_v2;
pub mod preset_parameter;

pub enum MeteoraDlmmAccount {
    BinArrayBitmapExtension(Box<bin_array_bitmap_extension::BinArrayBitmapExtension>),
    BinArray(Box<bin_array::BinArray>),
    LbPair(Box<lb_pair::LbPair>),
    Oracle(oracle::Oracle),
    Position(Box<position::Position>),
    PositionV2(Box<position_v2::PositionV2>),
    PresetParameter(preset_parameter::PresetParameter),
}

impl AccountDecoder<'_> for MeteoraDlmmDecoder {
    type AccountType = MeteoraDlmmAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            bin_array_bitmap_extension::BinArrayBitmapExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArrayBitmapExtension(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = bin_array::BinArray::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArray(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lb_pair::LbPair::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::LbPair(Box::new(decoded_account)),
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
                data: MeteoraDlmmAccount::Position(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position_v2::PositionV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PositionV2(Box::new(decoded_account)),
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
