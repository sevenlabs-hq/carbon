use {
    super::TokenMessengerMinterV2Decoder,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod denylisted_account;
pub mod local_token;
pub mod message_transmitter;
pub mod remote_token_messenger;
pub mod token_messenger;
pub mod token_minter;
pub mod token_pair;

pub enum TokenMessengerMinterV2Account {
    DenylistedAccount(denylisted_account::DenylistedAccount),
    LocalToken(local_token::LocalToken),
    MessageTransmitter(message_transmitter::MessageTransmitter),
    RemoteTokenMessenger(remote_token_messenger::RemoteTokenMessenger),
    TokenMessenger(token_messenger::TokenMessenger),
    TokenMinter(token_minter::TokenMinter),
    TokenPair(token_pair::TokenPair),
}

impl AccountDecoder<'_> for TokenMessengerMinterV2Decoder {
    type AccountType = TokenMessengerMinterV2Account;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            denylisted_account::DenylistedAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMessengerMinterV2Account::DenylistedAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = local_token::LocalToken::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMessengerMinterV2Account::LocalToken(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            message_transmitter::MessageTransmitter::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMessengerMinterV2Account::MessageTransmitter(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            remote_token_messenger::RemoteTokenMessenger::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMessengerMinterV2Account::RemoteTokenMessenger(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            token_messenger::TokenMessenger::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMessengerMinterV2Account::TokenMessenger(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            token_minter::TokenMinter::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMessengerMinterV2Account::TokenMinter(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_pair::TokenPair::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMessengerMinterV2Account::TokenPair(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
