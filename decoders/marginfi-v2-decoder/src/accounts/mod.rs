use {
    super::MarginfiV2Decoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod bank;
pub mod marginfi_account;
pub mod marginfi_group;

pub enum MarginfiV2Account {
    MarginfiAccount(Box<marginfi_account::MarginfiAccount>),
    MarginfiGroup(Box<marginfi_group::MarginfiGroup>),
    Bank(Box<bank::Bank>),
}

impl AccountDecoder<'_> for MarginfiV2Decoder {
    type AccountType = MarginfiV2Account;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }
        if let Some(decoded_account) =
            marginfi_account::MarginfiAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarginfiV2Account::MarginfiAccount(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            marginfi_group::MarginfiGroup::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarginfiV2Account::MarginfiGroup(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = bank::Bank::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarginfiV2Account::Bank(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
