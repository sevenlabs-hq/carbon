use {
    super::KaminoLendingDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod lending_market;
pub mod obligation;
pub mod referrer_state;
pub mod referrer_token_state;
pub mod reserve;
pub mod short_url;
pub mod user_metadata;
pub mod user_state;

pub enum KaminoLendingAccount {
    UserState(Box<user_state::UserState>),
    LendingMarket(Box<lending_market::LendingMarket>),
    Obligation(Box<obligation::Obligation>),
    ReferrerState(referrer_state::ReferrerState),
    ReferrerTokenState(Box<referrer_token_state::ReferrerTokenState>),
    ShortUrl(short_url::ShortUrl),
    UserMetadata(Box<user_metadata::UserMetadata>),
    Reserve(Box<reserve::Reserve>),
}

impl AccountDecoder<'_> for KaminoLendingDecoder {
    type AccountType = KaminoLendingAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = user_state::UserState::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::UserState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            lending_market::LendingMarket::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::LendingMarket(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = obligation::Obligation::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::Obligation(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            referrer_state::ReferrerState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::ReferrerState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            referrer_token_state::ReferrerTokenState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::ReferrerTokenState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = short_url::ShortUrl::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::ShortUrl(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            user_metadata::UserMetadata::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::UserMetadata(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = reserve::Reserve::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoLendingAccount::Reserve(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
