use {
    crate::TokenProgramDecoder,
    carbon_core::account::{AccountDecoder, DecodedAccount},
    solana_account::ReadableAccount,
    solana_program::program_pack::Pack,
};

pub enum TokenProgramAccount {
    Account(spl_token_interface::state::Account),
    Mint(spl_token_interface::state::Mint),
    Multisig(spl_token_interface::state::Multisig),
}

impl AccountDecoder<'_> for TokenProgramDecoder {
    type AccountType = TokenProgramAccount;

    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&spl_token_interface::id()) {
            return None;
        }

        if let Ok(data) = spl_token_interface::state::Account::unpack(account.data()) {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Account(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Ok(data) = spl_token_interface::state::Mint::unpack(account.data()) {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Mint(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Ok(data) = spl_token_interface::state::Multisig::unpack(account.data()) {
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
