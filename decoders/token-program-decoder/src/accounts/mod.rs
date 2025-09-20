use {
    crate::TokenProgramDecoder,
    carbon_core::account::{AccountDecoder, DecodedAccount},
    solana_account::ReadableAccount,
    spl_token::solana_program::program_pack::Pack,
};

pub enum TokenProgramAccount {
    Account(spl_token::state::Account),
    Mint(spl_token::state::Mint),
    Multisig(spl_token::state::Multisig),
}

impl AccountDecoder<'_> for TokenProgramDecoder {
    type AccountType = TokenProgramAccount;

    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<DecodedAccount<Self::AccountType>> {
        let spl_token_id = carbon_legacy::pubkey::to_modern(&spl_token::ID);

        if !account.owner.eq(&spl_token_id) {
            return None;
        }

        if let Ok(data) = spl_token::state::Account::unpack(account.data()) {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Account(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Ok(data) = spl_token::state::Mint::unpack(account.data()) {
            return Some(DecodedAccount {
                data: TokenProgramAccount::Mint(data),
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        };
        if let Ok(data) = spl_token::state::Multisig::unpack(account.data()) {
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
