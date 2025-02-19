use {
    super::OpenbookV2Decoder,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod book_side;
pub mod event_heap;
pub mod market;
pub mod open_orders_account;
pub mod open_orders_indexer;
pub mod stub_oracle;

pub enum OpenbookV2Account {
    Market(market::Market),
    OpenOrdersAccount(open_orders_account::OpenOrdersAccount),
    OpenOrdersIndexer(open_orders_indexer::OpenOrdersIndexer),
    StubOracle(stub_oracle::StubOracle),
    BookSide(book_side::BookSide),
    EventHeap(event_heap::EventHeap),
}

impl AccountDecoder<'_> for OpenbookV2Decoder {
    type AccountType = OpenbookV2Account;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = market::Market::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::Market(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            open_orders_account::OpenOrdersAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::OpenOrdersAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            open_orders_indexer::OpenOrdersIndexer::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::OpenOrdersIndexer(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = stub_oracle::StubOracle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::StubOracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = book_side::BookSide::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::BookSide(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = event_heap::EventHeap::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::EventHeap(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
