use rust_decimal::Decimal;
use spl_token::state::{Account, AccountState};

pub struct DBTokenAccount {
    pub mint: Vec<u8>,
    pub owner: Vec<u8>,
    pub amount: Decimal,
    pub delegate: Option<Vec<u8>>,
    pub state: AccountState,
    pub is_native: Option<Decimal>,
    pub delegated_amount: Decimal,
    pub close_authority: Option<Vec<u8>>,
}

impl From<Account> for DBTokenAccount {
    fn from(account: Account) -> Self {
        Self {
            mint: account.mint.to_bytes().to_vec(),
            owner: account.owner.to_bytes().to_vec(),
            amount: Decimal::from(account.amount),
            delegate: account.delegate.map(|d| d.to_bytes().to_vec()).into(),
            state: account.state,
            is_native: account.is_native.map(|n| Decimal::from(n)).into(),
            delegated_amount: Decimal::from(account.delegated_amount),
            close_authority: account
                .close_authority
                .map(|ca| ca.to_bytes().to_vec())
                .into(),
        }
    }
}
