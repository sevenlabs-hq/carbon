use carbon_gql_server::types::{amount::Amount, decimals::Decimals, pubkey::Pubkey};
use juniper::{GraphQLEnum, GraphQLObject};
use spl_token::{
    solana_program::program_option::COption,
    state::{Account, AccountState, Mint},
};

#[derive(GraphQLObject)]
pub struct GQLMint {
    pub mint_authority: Option<Pubkey>,
    pub supply: Amount,
    pub decimals: Decimals,
    pub is_initialized: bool,
    pub freeze_authority: Option<Pubkey>,
}

#[derive(GraphQLEnum)]
pub enum GQLAccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

#[derive(GraphQLObject)]
pub struct GQLAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: Amount,
    pub delegate: Option<Pubkey>,
    pub state: GQLAccountState,
    pub is_native: Option<Amount>,
    pub delegated_amount: Amount,
    pub close_authority: Option<Pubkey>,
}

impl From<Mint> for GQLMint {
    fn from(mint: Mint) -> Self {
        Self {
            mint_authority: match mint.mint_authority {
                COption::Some(mint_authority) => Some(Pubkey(mint_authority)),
                COption::None => None,
            },
            supply: Amount(mint.supply),
            decimals: Decimals(mint.decimals),
            is_initialized: mint.is_initialized,
            freeze_authority: match mint.freeze_authority {
                COption::Some(freeze_authority) => Some(Pubkey(freeze_authority)),
                COption::None => None,
            },
        }
    }
}

impl From<AccountState> for GQLAccountState {
    fn from(account_state: AccountState) -> Self {
        match account_state {
            AccountState::Uninitialized => Self::Uninitialized,
            AccountState::Initialized => Self::Initialized,
            AccountState::Frozen => Self::Frozen,
        }
    }
}

impl From<Account> for GQLAccount {
    fn from(account: Account) -> Self {
        Self {
            mint: Pubkey(account.mint),
            owner: Pubkey(account.owner),
            amount: Amount(account.amount),
            delegate: match account.delegate {
                COption::Some(delegate) => Some(Pubkey(delegate)),
                COption::None => None,
            },
            state: account.state.into(),
            is_native: match account.is_native {
                COption::Some(is_native) => Some(Amount(is_native)),
                COption::None => None,
            },
            delegated_amount: Amount(account.delegated_amount),
            close_authority: match account.close_authority {
                COption::Some(close_authority) => Some(Pubkey(close_authority)),
                COption::None => None,
            },
        }
    }
}
