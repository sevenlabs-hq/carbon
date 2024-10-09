use carbon_core::account::{AccountDecoder, DecodedAccount};
use solana_sdk::{account::ReadableAccount, program_pack::Pack};

use crate::TokenProgramDecoder;

pub enum TokenProgramAccount {
    Account(spl_token::state::Account),
    Mint(spl_token::state::Mint),
    Multisig(spl_token::state::Multisig),
}

impl<'a> AccountDecoder<'a> for TokenProgramDecoder {
    type AccountType = TokenProgramAccount;

    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner() != &spl_token::id() {
            return None;
        }

        if let Some(data) = spl_token::state::Account::unpack(account.data()).ok() {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Account(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Some(data) = spl_token::state::Mint::unpack(account.data()).ok() {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Mint(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Some(data) = spl_token::state::Multisig::unpack(account.data()).ok() {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Multisig(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };

        None
    }
}
