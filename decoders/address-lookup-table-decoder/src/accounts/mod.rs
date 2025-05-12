use {
    super::AddressLookupTableDecoder,
    crate::{
        types::{LookupTableAddresses, ProgramState},
        PROGRAM_ID,
    },
    address_lookup_table::AddressLookupTable,
    carbon_core::account::AccountDecoder,
    solana_pubkey::Pubkey,
};
pub mod address_lookup_table;

#[derive(Debug)]
pub enum AddressLookupTableAccount {
    AddressLookupTable(address_lookup_table::AddressLookupTable),
}

impl AccountDecoder<'_> for AddressLookupTableDecoder {
    type AccountType = AddressLookupTableAccount;

    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        let (meta, raw_data) = account.data.split_at(56);
        let program_data: ProgramState = bincode::deserialize(meta).ok()?;
        let ProgramState::LookupTable(meta) = program_data else {
            return None;
        };

        let addresses: &[Pubkey] = bytemuck::try_cast_slice(raw_data).ok()?;

        Some(carbon_core::account::DecodedAccount {
            lamports: account.lamports,
            data: AddressLookupTableAccount::AddressLookupTable(AddressLookupTable {
                meta,
                addresses: LookupTableAddresses {
                    addresses: addresses.to_vec(),
                },
            }),
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        })
    }
}
