use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PhoenixDecoder;
pub mod market_header;
pub mod seat;

pub enum PhoenixAccount {
    MarketHeader(Box<market_header::MarketHeader>),
    Seat(seat::Seat),
}

impl AccountDecoder<'_> for PhoenixDecoder {
    type AccountType = PhoenixAccount;
    fn decode_account(
        &self,
        account: &'_ solana_account::Account,
        _metadata: Option<&carbon_core::account::AccountMetadata>,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            market_header::MarketHeader::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PhoenixAccount::MarketHeader(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = seat::Seat::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PhoenixAccount::Seat(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
