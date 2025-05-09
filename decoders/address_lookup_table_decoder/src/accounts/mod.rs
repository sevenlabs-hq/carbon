use {
    super::AddressLookupTableDecoder,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod address_lookup_table;
pub mod uninitialized;

pub enum AddressLookupTableAccount {
    Uninitialized(uninitialized::Uninitialized),
    AddressLookupTable(address_lookup_table::AddressLookupTable),
}

impl AccountDecoder<'_> for AddressLookupTableDecoder {
    type AccountType = AddressLookupTableAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            uninitialized::Uninitialized::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AddressLookupTableAccount::Uninitialized(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            address_lookup_table::AddressLookupTable::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AddressLookupTableAccount::AddressLookupTable(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
