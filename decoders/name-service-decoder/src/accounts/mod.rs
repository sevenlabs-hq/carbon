use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::NameDecoder;
pub mod name_record_header;

pub enum NameAccount {
    NameRecordHeader(name_record_header::NameRecordHeader),
}

impl<'a> AccountDecoder<'a> for NameDecoder {
    type AccountType = NameAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            name_record_header::NameRecordHeader::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: NameAccount::NameRecordHeader(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
