use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use solana_pubkey::Pubkey;
use spl_token_interface::state::{Account, AccountState, Mint};

#[derive(sqlx::Type)]
#[sqlx(type_name = "account_state")]
pub enum DBAccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

impl From<AccountState> for DBAccountState {
    fn from(account_state: AccountState) -> Self {
        match account_state {
            AccountState::Uninitialized => Self::Uninitialized,
            AccountState::Initialized => Self::Initialized,
            AccountState::Frozen => Self::Frozen,
        }
    }
}

impl From<DBAccountState> for AccountState {
    fn from(db_account_state: DBAccountState) -> Self {
        match db_account_state {
            DBAccountState::Uninitialized => Self::Uninitialized,
            DBAccountState::Initialized => Self::Initialized,
            DBAccountState::Frozen => Self::Frozen,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct DBTokenAccount {
    pub mint: Vec<u8>,
    pub owner: Vec<u8>,
    pub amount: Decimal,
    pub delegate: Option<Vec<u8>>,
    pub state: DBAccountState,
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
            state: account.state.into(),
            is_native: account.is_native.map(Decimal::from).into(),
            delegated_amount: Decimal::from(account.delegated_amount),
            close_authority: account
                .close_authority
                .map(|ca| ca.to_bytes().to_vec())
                .into(),
        }
    }
}

impl TryFrom<DBTokenAccount> for Account {
    type Error = String;

    fn try_from(db_token: DBTokenAccount) -> Result<Self, Self::Error> {
        Ok(Self {
            mint: Pubkey::try_from(db_token.mint).map_err(|_| "parse mint".to_string())?,
            owner: Pubkey::try_from(db_token.owner).map_err(|_| "parse owner".to_string())?,
            amount: db_token.amount.to_u64().ok_or("amount to u64")?,
            delegate: db_token
                .delegate
                .map(|d| {
                    Pubkey::try_from(d)
                        .map_err(|_| "parse delegate".to_string())
                        .expect("Failed to parse delegate")
                })
                .into(),
            state: db_token.state.into(),
            is_native: db_token
                .is_native
                .map(|n| n.to_u64().expect("Failed to parse is native"))
                .into(),
            delegated_amount: db_token
                .delegated_amount
                .to_u64()
                .ok_or("delegated amount to u64")?,
            close_authority: db_token
                .close_authority
                .map(|ca| {
                    Pubkey::try_from(ca)
                        .map_err(|_| "parse close authority".to_string())
                        .expect("Failed to parse close authority")
                })
                .into(),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct DBMint {
    pub mint_authority: Option<Vec<u8>>,
    pub supply: Decimal,
    pub decimals: i16,
    pub is_initialized: bool,
    pub freeze_authority: Option<Vec<u8>>,
}

impl From<Mint> for DBMint {
    fn from(mint: Mint) -> Self {
        Self {
            supply: Decimal::from(mint.supply),
            mint_authority: mint.mint_authority.map(|d| d.to_bytes().to_vec()).into(),
            decimals: mint.decimals as i16,
            is_initialized: mint.is_initialized,
            freeze_authority: mint.freeze_authority.map(|d| d.to_bytes().to_vec()).into(),
        }
    }
}

impl TryFrom<DBMint> for Mint {
    type Error = String;

    fn try_from(db_mint: DBMint) -> Result<Self, Self::Error> {
        Ok(Self {
            supply: db_mint.supply.to_u64().ok_or("supply to u64")?,
            mint_authority: db_mint
                .mint_authority
                .map(|d| {
                    Pubkey::try_from(d)
                        .map_err(|_| "parse mint authority".to_string())
                        .expect("Failed to parse mint authority")
                })
                .into(),
            decimals: db_mint.decimals as u8,
            is_initialized: db_mint.is_initialized,
            freeze_authority: db_mint
                .freeze_authority
                .map(|d| {
                    Pubkey::try_from(d)
                        .map_err(|_| "parse freeze authority".to_string())
                        .expect("Failed to parse freeze authority")
                })
                .into(),
        })
    }
}
