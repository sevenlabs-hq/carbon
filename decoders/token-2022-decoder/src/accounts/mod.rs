use {
    super::Token2022Decoder,
    carbon_core::account::AccountDecoder,
    solana_account::ReadableAccount,
    solana_program_pack::Pack,
    spl_token_2022::{
        extension::StateWithExtensions,
        state::{Account as TokenAccount, Mint, Multisig},
    },
};

pub enum Token2022Account<'data> {
    Mint(StateWithExtensions<'data, Mint>),
    Token(StateWithExtensions<'data, TokenAccount>),
    Multisig(Multisig),
}

impl<'data> AccountDecoder<'data> for Token2022Decoder {
    type AccountType = Token2022Account<'data>;

    fn decode_account(
        &self,
        account: &'data solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&spl_token_2022::id()) {
            return None;
        }

        if let Ok(decoded_account) = StateWithExtensions::<Mint>::unpack(account.data()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: Token2022Account::Mint(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Ok(decoded_account) = StateWithExtensions::<TokenAccount>::unpack(account.data()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: Token2022Account::Token(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Ok(decoded_account) = Multisig::unpack(account.data()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: Token2022Account::Multisig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
